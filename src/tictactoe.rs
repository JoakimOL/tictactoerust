use std::io;

#[derive(Debug)]
struct Board {
    width: u8,
    height: u8,
    data: Vec<char>,
}

impl Board {
    fn print(&self) {
        let mut i = 0;
        for val in &self.data {
            print!("| {} ", val);
            i += 1;
            if i % self.width == 0 {
                println!("|");
            }
        }
    }

    fn update(&mut self, x: u8, y: u8, turn: bool) {
        let sym = if turn { 'o' } else { 'x' };
        self.data[(x + y * self.height) as usize] = sym;
    }

    fn check(&self) -> (bool, String) {
        println!("");
        (true, String::from("x won!"))
    }
}

const DIM: u8 = 3;

fn main() {
    let mut turn = true;
    let mut board = create_board(DIM, DIM);
    loop {
        board.print();
        let player = if turn { 1 } else { 2 };

        let xinput = clamp(
            0,
            DIM,
            take_input(format!("player {}, please enter x", player)) - 1,
        );
        let yinput = clamp(
            0,
            DIM,
            take_input(format!("player {}, please enter y", player)) - 1,
        );

        println!("x: {}", xinput);
        println!("y: {}", yinput);
        board.update(xinput, yinput, turn);
        let res: (bool, String) = board.check();
        if res.0 == true {
            println!("{}", res.1);
            board.print();
            break;
        }
        // board.print();
        turn = !turn;
    }
}

fn clamp(min: u8, max: u8, val: u8) -> u8 {
    match val {
        val if val > max => max,
        val if val < min => min,
        val => val,
    }
}

fn check_input(input: i8) -> Result<u8, String> {
    if input > DIM as i8 || 0 > input {
        return Err(String::from("input out of bounds. try again"));
    }
    Ok(input as u8)
}

/*
 * Takes user input as a unsigned 8bit int.
 * If the user supplies something that is not a number,
 * the loop continues and the user is prompted again.
 *
 * Need a more elegant solution to the while loop conditional
 * How can i make it loop till the user successfully input a number?
 */
fn take_input(prompt: String) -> u8 {
    let mut input: i8 = -1;
    while input == -1 {
        println!("{}", prompt);
        let mut inputstr = String::new();
        io::stdin()
            .read_line(&mut inputstr)
            .expect("Failed to read line! {}");

        input = match inputstr.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    match check_input(input) {
        Ok(n) => n as u8,
        Err(msg) => take_input(msg),
    }
}

/*
 * Creates a new board with the given dimensions.
 * Fills it with dashes
 *
 * Is it possible to fill a dynamic length vector with values
 * without using a separate for loop?
 * something equivalent of the static length version
 * vec!['-';width]
 */
fn create_board(width: u8, height: u8) -> Board {
    let mut res = Board {
        width,
        height,
        data: vec![],
    };
    for _ in 0..width * height {
        res.data.push('-');
    }
    res
}
