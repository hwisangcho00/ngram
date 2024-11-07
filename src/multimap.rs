use std::borrow::Borrow;
use std::collections::{hash_map::DefaultHasher, LinkedList};
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

// The ConcurrentMultiMap struct is a concurrent hash map that allows multiple values to be
// associated with a single key. It is implemented using a vector of RwLocks, where each lock
// protects a linked list of key-value pairs.
pub struct ConcurrentMultiMap<K: Hash + Eq, V> {
    buckets: Vec<RwLock<LinkedList<(K, V)>>>,
}

impl<K: Hash + Eq, V> ConcurrentMultiMap<K, V> {
    // Create a new empty ConcurrentMultiMap with the given number of buckets.
    pub fn new(bucket_count: usize) -> Self {
        
        let mut buckets = Vec::with_capacity(bucket_count);

        for _i in 0..bucket_count {
            let new_bucket = RwLock::new(LinkedList::new());
            buckets.push(new_bucket);
        }

        ConcurrentMultiMap {
            buckets
        }

    }
}

impl<K: Hash + Eq, V: Clone + Eq> ConcurrentMultiMap<K, V> {
    // TODO:
    // Associate the given value with the given key. To do so, hash the key, and find the
    // corresponding bucket in the vector by modulo-ing the hash by the number of buckets. Then,
    // take a writer lock of the bucket and iterate over the linked list, checking if the
    // key-values pair already exists. If it does, return early. Otherwise, add the key-value pair
    // to the linked list.
    pub fn set(&self, key: K, value: V) {
        
        let hash_val = hash_key(&key);
        let mod_val = (hash_val % (self.buckets.len() as u64)) as usize;

        if let Some(bucket) = self.buckets.get(mod_val) {
            
            let mut bucket = bucket.write().unwrap();

            for (existing_key, existing_value) in bucket.iter() {
                if *existing_key == key && *existing_value == value{
                    return;
                }
            }

            bucket.push_back((key, value));

        }

    }

    // TODO:
    // Retrieve all values associated with `key`. To do so, hash the key, and find the
    // corresponding bucket in the vector by modulo-ing the hash by the number of buckets. Then,
    // take a reader lock of the bucker and iterate over the linked list, collecting all values
    // associated with the key by `clone`-ing them. Return the collected values.
    pub fn get<Q>(&self, key: &Q) -> Vec<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut result = Vec::new();

        let hash_val = hash_key(&key);
        let mod_val = (hash_val % (self.buckets.len() as u64)) as usize;

        if let Some(bucket) = self.buckets.get(mod_val) {

            let bucket = bucket.read().unwrap();

            for (existing_key, value) in bucket.iter() {
                if existing_key.borrow() == key {
                    result.push(value.clone());
                }
                
            }

        }

        result

    }
}

fn hash_key<K : Hash> (key: &K) -> u64 {

    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}
