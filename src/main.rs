mod instruction;
mod processor;
mod register;
mod stack;
mod program;

fn main() {
    let mut _processor = processor::Processor::<u16, u16, 1024>::new();
}
