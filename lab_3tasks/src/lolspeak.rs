pub fn lolspeak(&mut self) {
    self.noun_phrase();
    self.verb();
    self.noun_phrase();
}
pub fn noun_phrase(&mut self){
    self.article();
    self.adjective();
    self.noun();
}
pub fn verb(&mut self){
// is my current token a verb
    if self.lexer.is_a_verb(&self.compiler.current_token) {
        self.next_token();
    } else {
        eprintln!("a syntax error was encountered.");
    // exit
    std::process::exit(1);
    }
}