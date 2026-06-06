use std::fmt::{Display, Formatter};
use fixed::FixedSizeArray;

// A SlowVec contains a fixed number of elements.
// The elements are of type "T"
// This is Rust's way of saying that SlowVec can accept any type for the elements.
// E.g., SlowVec<i32> represents a SlowVec with integer elements,
//       SlowVec<bool> represents a SlowVec with bool elements,
//       etc.
// look at main.rs for an example.
pub struct SlowVec<T> {
    fixed: FixedSizeArray<T>, //slowvecs are a fixed size array 
}

// Functions inside SlowVec.
impl<T> SlowVec<T> {
    pub fn new() -> Self {
        return SlowVec {
            fixed: FixedSizeArray::allocate(0)
        };
    }
    
    // returns the length of the SlowVec.
    pub fn len(&self) -> usize {
        return self.fixed.len();
    }

    // Transforms an instance of SlowVec to a regular vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.fixed.len()); //make a new vec with the size of the SlowVec
        for i in 0..self.fixed.len() {
            v.push(self.fixed.move_out(i)); //push elements of SlowVec into new vec
        }
        v
    }

    // Transforms a vector to a SlowVec.
    pub fn from_vec(vec: Vec<T>) -> SlowVec<T> {
        let mut tmp = FixedSizeArray::allocate(vec.len()); //create a new SlowVec
        let mut index = 0; 
        for element in vec {
            tmp.put(element, index); //put element into new Slowvec
            index = index + 1; //index as counter
        }
        return SlowVec { fixed: tmp };
    }

    // Clear the content of this vector.
    pub fn clear(&mut self) {
        self.fixed = FixedSizeArray::allocate(0); //equate current SlowVec with new slowvec of len 0
    }

    // Get a reference to the element at position i.
    // Think of a reference as a read-only "copy" of the element.
    // We will talk about what references are in class.
    // Note: the element remains stored in the SlowVec after get(). It is not removed.
    pub fn get(&self, i: usize) -> &T {
        self.fixed.get(i) //fixed size array get returns a reference
    }

    pub fn push(&mut self, t: T) {
        //1. create new SlowVec with +1 len 
        let mut tmp: FixedSizeArray<T> = FixedSizeArray::allocate(self.fixed.len()+1);
        
        //2. push old elements from self into tmp 
        for i in 0..(self.fixed.len()) {
            tmp.put(self.fixed.move_out(i), i); //move_out b/c tmp needs to own elements, but elements currently owned by self.fixed
        }
        //3. add new element T into tmp 
        tmp.put(t, self.fixed.len());
        self.fixed = tmp; //update it
    }

    pub fn remove(& mut self, i: usize) {
        //1. create new SlowVec with -1 len
        let mut tmp = FixedSizeArray::allocate(self.fixed.len()-1);

        //2. push old elements from self into tmp from 0 up to index
        for j in 0..i {
            tmp.put(self.fixed.move_out(j), j);  //move out b/c 
        }
        //3. skip adding old element at index and continue from after that index
        for j in (i+1)..self.fixed.len() {
            tmp.put(self.fixed.move_out(j), j-1); 
        }
        self.fixed = tmp;
    }
}


// This allows us to print the SlowVec using println!().
impl<T: Display> Display for SlowVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlowVec({})", self.fixed)
    }
}
