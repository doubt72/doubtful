// Evaluate parsed stuff

use encoding::Block;

// Look how simple this is!  ...Because we hid all of the logic in the types

pub fn evaluate(block: &Block) {
  let mut scope = Vec::new();
  block.evaluate(&mut scope);
}
