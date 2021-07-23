use console::Term;

pub fn construct_interface() -> String {
    let buffer: Term = Term::buffered_stdout();
    let (rows, cols) = buffer.size();

    const WALL: &str = "|";
    const CURSOR: &str = "█";
    const DASH: &str = "-";
    const NEW_LINE: char = '\n';
    const SPACE: &str = " ";

    /*
    |-------------------------|
    |                         |
    |                         |
    |                         |
    |                         |
    |                         |
    |                         |
    |                         |
    |-------------------------|
    | █                       |  █ - ASCII CODE 219
    |-------------------------|
    */

    let mut interface = String::new();
    //Does this: |-------------------------|
    let dashed_line: String = format!(
        "{0}{1}{0}",
        WALL,
        format!("{}", str::repeat(DASH, (cols - 2) as usize))
    );

    interface.push_str(dashed_line.as_str());
    interface.push(NEW_LINE);

    //Constructs the *currently empty* space between the typing space and the top
    //Real number is rows - 4, debugging - rows - 5 (stupid press any key bullshit)
    // for _i in 0..rows - 4 {
    for _i in 0..rows - 5 {
        let just_spaces = str::repeat(" ", (cols - 2) as usize);
        let the_space: String = format!("{0}{1}{0}{2}", WALL, just_spaces, NEW_LINE);
        interface.push_str(the_space.as_str());
    }

    interface.push_str(dashed_line.as_str());
    interface.push(NEW_LINE);

    //Construct typing space
    let typing_space: String = format!(
        "{0}{1}{2}{0}{3}",
        WALL,
        CURSOR,
        str::repeat(SPACE, (cols - 3) as usize),
        NEW_LINE
    );
    interface.push_str(typing_space.as_str());
    interface.push_str(dashed_line.as_str());

    return interface;
}
