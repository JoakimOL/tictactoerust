use std::io;

/*
 * Struct that describes the game board
 * contains its dimensions and data.
 */
#[derive(Debug)]
struct Board {
    width:  u8,
    height: u8,
    data:   Vec<Vec<char>>,
}

impl Board {
    /*
     * sadly, the win-detection (MAGIC, INIT and check)
     * was stolen from somewhere I cannot remember
     */
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const MAGIC: [u32; 9] = [
        0x10010010, 0x01010000, 0x00110001,
        0x10001000, 0x01001011, 0x00101000,
        0x10000101, 0x01000100, 0x00100110,
    ];

    const INIT: u32 = 0x11111111;

    /*
     * check for three in a row
     */
    fn check(&self, score: u32) -> bool {
        score & 0x44444444 > 0
    }

    /*
     * updates the board at the given coordinates with
     * an appropriate symbol
     */
    fn update(&mut self, x: u8, y: u8, turn: bool) -> Result<(), ()> {
        let sym = if turn { 'o' } else { 'x' };
        if self.data[y as usize][x as usize] == '-' {
            self.data[y as usize][x as usize] = sym;
            Ok(())
        }
        else {
            Err(())
        }
    }

    /*
     * pretty print function
     */
    fn print(&self) {
        for line in &self.data {
            for val in line {
                print!("| {} ", val);
            }
            println!("|");
        }
    }
}
// Board dimensions
const DIM: u8 = 3;

fn main() {
    let mut turn = true;
    let mut scores: [u32; 2] = [Board::INIT, Board::INIT];
    let mut board = create_board(DIM, DIM);

    loop {
        board.print();
        let player = if turn { 0 } else { 1 };

        let xinput = clamp(
            0,
            DIM,
            take_input(format!(
                "player {}, please enter x coordinate (1-3)",
                player + 1
            )) - 1,
        );
        let yinput = clamp(
            0,
            DIM,
            take_input(format!(
                "player {}, please enter y coordinate (1-3)",
                player + 1
            )) - 1,
        );

        println!("x: {}", xinput);
        println!("y: {}", yinput);

        // if the input is successfully registered - update scores
        // if not, let the user try again
        match board.update(xinput, yinput, turn) {
            Ok(_) => {
                scores[player] = scores[player]
                    + Board::MAGIC[(yinput + xinput * DIM) as usize];
            }
            Err(_) => {
                println!("Invalid input! Cell is taken.");
                continue;
            }
        }

        // game won!
        if board.check(scores[player]) {
            println!("player {} won!", player + 1);
            board.print();
            break;
        }
        turn = !turn;
    }
}

/*
 * clamps the value val between min and max
 */
fn clamp(min: u8, max: u8, val: u8) -> u8 {
    match val {
        val if val > max => max,
        val if val < min => min,
        val => val,
    }
}

/*
 *verifies that the input is within bounds
 */
fn check_input(input: i8) -> Result<u8, String> {
    if input > DIM as i8 || 0 >= input {
        return Err(String::from("input out of bounds. try again"));
    }
    Ok(input as u8)
}

/*
 * prompts user to input an integer.
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
 */
fn create_board(width: u8, height: u8) -> Board {
    let mut res = Board {
        width,
        height,
        data: vec![],
    };
    for _ in 0..height {
        let mut temp = vec![];
        for _ in 0..width {
            temp.push('-');
        }
        res.data.push(temp);
    }
    res
}
