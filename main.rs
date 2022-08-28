#[derive(Debug)]
enum IssueRequest {
    OPCODE,
}

impl From<u32> for IssueRequest {
    fn from(_: u32) -> IssueRequest {
        IssueRequest::OPCODE
    }
}

fn main() {
    let data = (&[], false);

    println!("{:?}", IssueRequest::OPCODE);

    let ir = 1.into();

    println!("{ir:?}");
}
