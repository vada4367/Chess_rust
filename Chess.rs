trait Figure
{
    fn init(x :u8, y :u8, _player :u8) -> Self where Self: Sized;
    fn player(&self) -> u8;

    fn figure(&self) -> String
    {
        return if self.figure() != ".".to_string() && self.player() == 1 { self.figure() } else { self.figure().to_lowercase() }
    }
}


pub struct PES
{
    x :u8,
    y :u8,
    figure :String,
    player :u8,
}

impl Figure for PES
{
    fn init(_x :u8, _y :u8, _player :u8) -> PES
    {
        PES{x : _x, y : _y, figure : "P".to_string(), player : _player}
    }
    
    fn player(&self) -> u8 { self.player }
}

pub struct KON
{
    x :u8,
    y :u8,
    figure :String,
    player :u8,
}

impl Figure for KON
{
    fn init(_x :u8, _y :u8, _player :u8) -> KON
    {
        KON{x : _x, y : _y, figure : "L".to_string(), player : _player}
    }

    fn player(&self) -> u8 { self.player }
}

pub struct NUL
{
    x :u8,
    y :u8,
    figure :String,
}

impl Figure for NUL
{
    fn init(_x :u8, _y :u8, _player :u8) -> NUL
    {
        NUL{x : _x, y : _y, figure : ".".to_string()}
    }

    fn player(&self) -> u8 { 2 }

    fn figure(&self) -> String
    {
        ".".to_string()
    }
}

pub struct Chess
{
    pub board :Vec<Box<dyn Figure>>,
    x :u8,
    y :u8,
}

impl Chess
{
    pub fn init(_X :u8, _Y :u8) -> Chess
    {
        let mut local_board :Vec<Box<dyn Figure>> = vec![];

        for x in 0.._X
        {
            for y in 0.._Y
            {
                local_board.push(Box::new(NUL::init(x as u8, y as u8, 1)));
            }
        }

        Chess{x : _X, y : _Y, board : local_board}
    }

    pub fn print(&self)
    {
        for y in 0..self.y
        {
            for x in 0..self.x
            {
                print!("{} ", self.board[(y * self.y + x) as usize].figure());
            }
            println!();
        }
    }

    pub fn init_figures(&mut self)
    {
        let mut y = 0u8;
        let mut x = 0u8;
        self.board[(y * self.y + x) as usize].figure = "r".to_string();
        self.board[(y * self.y + self.x - x - 1) as usize].figure = "r".to_string();
                        
        self.board[((self.y - y - 1) * self.y + x) as usize].figure = "R".to_string();
        self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = "R".to_string();

        x = 1;
        self.board[(y * self.y + x) as usize] = Box::new(KON::init(x, y, 0));
        self.board[(y * self.y + self.x - x - 1) as usize] = Box::new(KON::init(x, y, 0));

                        
        self.board[((self.y - y - 1) * self.y + x) as usize] = Box::new(KON::init(x, y, 1));
        self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize] = Box::new(KON::init(x, y, 1));

                    
        x = 2;
        self.board[(y * self.y + x) as usize].figure = "b".to_string();
        self.board[(y * self.y + self.x - x - 1) as usize].figure = "b".to_string();
                        
        self.board[((self.y - y - 1) * self.y + x) as usize].figure = "B".to_string();
        self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = "B".to_string();

        x = 3;
        self.board[(y * self.y + x) as usize].figure = "k".to_string();
        self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = "K".to_string();

        x = 4;
        self.board[(y * self.y + x) as usize].figure = "q".to_string();            
        self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = "Q".to_string();
        y = 1 ;
        for x in 0..self.x
        {
            self.board[(y * self.y + x) as usize] = Box::new(PES::init(x, y, 0));
            self.board[((self.y - y - 1) * self.y + x) as usize] = Box::new(KON::init(x, y, 1));
        }
    }
}
