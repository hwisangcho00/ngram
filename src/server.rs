use crate::database::Database;
use crate::message::*;
use crate::pool::ThreadPool;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

/// The number of workers in the server's thread pool
const WORKERS: usize = 16;

// TODO:
// Implement the `process_message` function. This function should take a `ServerState`, a `Request`,
// and a `TcpStream`. It should process the request and write the response to the stream.
// Processing the request should simply require calling the appropriate function on the database
// and then creating the appropriate response and turning it into bytes which are sent to along
// the stream by calling the `write_all` method.
fn process_message(state: Arc<ServerState>, request: Request, mut stream: TcpStream) {
    
    let response = match request {
        Request::Publish { doc } => {
            let doc_id = state.database.publish(doc);
            Response::PublishSuccess(doc_id)
        },
        Request::Search { word } => {
            let results = state.database.search(&word);
            Response::SearchSuccess(results)
        },
        Request::Retrieve { id } => {
            match state.database.retrieve(id) {
                Some(doc) => {
                    Response::RetrieveSuccess(doc)
                },
                None => Response::Failure
            }
        }
    };

    let response_bytes = response.to_bytes();

    if let Err(e) = stream.write_all(&response_bytes) {
        eprintln!("Failed to process message: {}", e);
    }

}

/// A struct that contains the state of the server
struct ServerState {
    /// The database that the server uses to store documents
    database: Database,
    /// The thread pool that the server uses to process requests
    pool: ThreadPool,
    /// A flag that indicates whether the server has been stopped
    is_stopped: AtomicBool,
}
impl ServerState {
    fn new() -> Self {
        Self {
            database: Database::new(),
            pool: ThreadPool::new(WORKERS),
            is_stopped: AtomicBool::new(false),
        }
    }
}

pub struct Server {
    state: Arc<ServerState>,
}

fn handle_connection(state: Arc<ServerState>, mut stream: TcpStream) {
    // Attempt to deserialize a request from the stream
    if let Some(request) = Request::from_bytes(&mut stream) {
        // Process the request if deserialization succeeds
        process_message(state, request, stream);
    } else {
        eprintln!("Failed to deserialize request");
    }
}

impl Server {
    // TODO:
    // Create a new server by using the `ServerState::new` function
    pub fn new() -> Self {
        let server_state = ServerState::new();
        let state = Arc::new(server_state);

        Server {
            state
        }

    }

    // TODO:
    // Spawn a thread that listens for incoming connections on the given port. When a connection is
    // established, add a task to the thread pool that deserializes the request, and processes it
    // using the `process_message` function.
    //
    // To listen for incoming connections, you can use the `std::net::TcpListener::bind` function.
    // To listen on the local address, you can call `TcpListener::bind(("127.0.0.1", port))`. The
    // resulting TcpListener can be used to accept incoming connections by calling the `accept`
    // method in a loop. This method blocks until a new connection is established, and then returns
    // a new TcpStream and the address of the remote peer. You should move this stream into the
    // task that you send to the thread pool.
    //
    // While looping to accept connections, you should also check the `is_stopped` flag in the
    // `ServerState` to see if the server has been stopped. If it has, you should break out of the
    // loop and return.

    fn listen(&self, port: u16) {
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            let listener = TcpListener::bind(("127.0.0.1", port)).expect("Failed to bind to address");
            println!("Server listening on port {}", port);

            loop {
                if state.is_stopped.load(Ordering::SeqCst) {
                    println!("Server is stopping...");
                    break;
                }
                match listener.accept() {
                    Ok((stream, _addr)) => {
                        let state_clone = Arc::clone(&state);

                        state.pool.execute(move || {
                            handle_connection(state_clone, stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
    }



    // This function has already been partially completed for you
    pub fn run(&self, port: u16) {
        // Set up a signal handler to stop the server when Ctrl-C is pressed
        let state = Arc::clone(&self.state);
        match ctrlc::try_set_handler(move || {
            println!("Stopping server...");
            state.is_stopped.store(true, Ordering::SeqCst);
        }) {
            Ok(_) => {}
            Err(ctrlc::Error::MultipleHandlers) => {}
            Err(e) => {
                panic!("Error setting Ctrl-C handler: {}", e);
            }
        }

        // TODO: Call the listen function and then loop (doing nothing) until the server has been stopped

        self.listen(port);

        while !self.state.is_stopped.load(Ordering::SeqCst) {

        }
    }
    pub fn stop(&self) {
        self.state.is_stopped.store(true, Ordering::SeqCst);
    }
}
