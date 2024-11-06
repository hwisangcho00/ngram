/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Use hex 0x01, 0x02, 0x03 to distinguish between different request
        match self {
            Request::Publish { doc} => {
                bytes.push(0x01);
                bytes.extend((doc.len() as usize).to_be_bytes());
                bytes.extend(doc.as_bytes());

            },
            Request::Search { word } => {
                bytes.push(0x02);
                bytes.extend((word.len() as usize).to_be_bytes());
                bytes.extend(word.as_bytes());

            },
            Request::Retrieve { id } => {
                bytes.push(0x03);
                bytes.extend(id.to_be_bytes());
            }
        }

        bytes

    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag = [0;1];
        reader.read_exact(&mut tag).ok()?;

        let mut len_buffer = [0;8];

        match tag[0] {
            0x01 => {
                reader.read_exact(&mut len_buffer).unwrap();

                let len = usize::from_be_bytes(len_buffer);

                let mut doc_buffer = vec![0;len];

                reader.read_exact(&mut doc_buffer).unwrap();

                let doc = String::from_utf8(doc_buffer).unwrap();

                Some(Request::Publish {doc})
                
            },
            0x02 => {
                reader.read_exact(&mut len_buffer).unwrap();

                let len = usize::from_be_bytes(len_buffer);

                let mut word_buffer = vec![0;len];

                reader.read_exact(&mut word_buffer).unwrap();

                let word = String::from_utf8(word_buffer).unwrap();

                Some(Request::Search {word})
                
            },
            0x03 => {
                reader.read_exact(&mut len_buffer).unwrap();

                let id = usize::from_be_bytes(len_buffer);

                Some(Request::Retrieve {id})
                
            },
            _ => None,
        }


    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Use hex 0x01, 0x02, 0x03 to distinguish between different request
        match self {
            Response::PublishSuccess (index) => {
                bytes.push(0x01);

                bytes.extend(index.to_be_bytes());

            },
            Response::SearchSuccess(indices) => {
                bytes.push(0x02);

                bytes.extend((indices.len() as usize).to_be_bytes());

                for &index in indices {
                    bytes.extend(index.to_be_bytes());
                }
            
            },
            Response::RetrieveSuccess(doc) => {
                bytes.push(0x03);

                bytes.extend((doc.len() as usize).to_be_bytes());

                bytes.extend(doc.as_bytes());
            }
            Response::Failure => {
                bytes.push(0x04);
            }
        }

        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag = [0; 1];
        reader.read_exact(&mut tag).unwrap();

        match tag[0] {
            0x01 => {
                let mut index_buffer = [0; 8];

                reader.read_exact(&mut index_buffer).unwrap();

                let index = usize::from_be_bytes(index_buffer);

                Some(Response::PublishSuccess(index))
            },
            0x02 => {
                let mut len_buffer = [0; 8];

                reader.read_exact(&mut len_buffer).unwrap();
                
                let len = usize::from_be_bytes(len_buffer);

                let mut indices = Vec::with_capacity(len);

                for _ in 0..len {
                    let mut index_buffer = [0; 8];
                    reader.read_exact(&mut index_buffer).unwrap();
                    indices.push(usize::from_be_bytes(index_buffer));
                }
                
                Some(Response::SearchSuccess(indices))
            },
            0x03 => {
                let mut len_buffer = [0; 8];

                reader.read_exact(&mut len_buffer).unwrap();

                let len = usize::from_be_bytes(len_buffer);

                let mut doc_buffer = vec![0; len];

                reader.read_exact(&mut doc_buffer).unwrap();
                
                let doc = String::from_utf8(doc_buffer).unwrap();
                
                Some(Response::RetrieveSuccess(doc))
            },

            0x04 => Some(Response::Failure), 

            _ => None, 
        }



    }
}

