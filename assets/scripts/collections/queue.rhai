
fn create() {
  #{ first: 0, last: -1 }
}

fn push_left(value) {
  let first = this.first - 1;
  this.first = first;
  this[first] = value;
}

fn push_right(value) {
  let last = this.last + 1;
  this.last = last;
  this[last] = value;
}

fn peek_left() {
  this[this.first]
}

fn peek_right() {
  this[this.last]
}

fn pop_left() {
  let first = this.first;
  
  if this.last < first {
    return ();
  }
  
  let value = this[first];
  this[first] = ();
  this.first = first + 1;
  value
}

fn pop_right() {
  let last = this.last;
  
  if last < this.first {
    return ();
  }
  
  let value = this[last];
  this[last] = ();
  this.last = last - 1;
  value
}
