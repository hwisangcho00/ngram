use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::server::Server;

// TODO:
// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Client {
        server_address: String,
        server_port: u16,
        #[command(subcommand)]
        action: ClientActions,
    },
    Server {
        listen_port: u16,
    },
}

#[derive(Subcommand, Debug)]
enum ClientActions {
    Publish {
        document_path: String,
    },
    Search {
        word: String,
    },
    Retrieve {
        document_id: usize,
    },
}
// TODO:
// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Client {
            server_address,
            server_port,
            action,
        } => {
            // Instantiate the client
            let client = Client::new(&server_address, server_port);

            match action {
                ClientActions::Publish { document_path } => {
                    println!("Publishing document at: {}", document_path);
                    match client.publish_from_path(&document_path) {
                        Some(response) => println!("Response: {:?}", response),
                        None => eprintln!("Failed to publish document"),
                    }
                }
                ClientActions::Search { word } => {
                    println!("Searching for word: {}", word);
                    match client.search(&word) {
                        Some(response) => println!("Response: {:?}", response),
                        None => eprintln!("Failed to search for the word"),
                    }
                }
                ClientActions::Retrieve { document_id } => {
                    println!("Retrieving document with ID: {}", document_id);
                    match client.retrieve(document_id) {
                        Some(response) => println!("Response: {:?}", response),
                        None => eprintln!("Failed to retrieve document"),
                    }
                }
            }
        }
        Commands::Server { listen_port } => {
            println!("Starting server on port: {}", listen_port);
            let server = Server::new();  

            server.run(listen_port);
        }
    }
}
