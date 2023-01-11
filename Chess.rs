pub struct Figure
{
    figure :String,
}

impl Figure
{
    pub fn null_init() -> Figure
    {
        Figure{figure : ".".to_string()}
    }
}

pub struct Chess
{
    pub board :Vec<Figure>,
    x :u8,
    y :u8,
}

impl Chess
{
    pub fn init(x :u8, y :u8) -> Chess
    {
        if x < 1 || y < 1 { return Chess{
                                board : vec![Figure{figure : "".to_string()}], 
                                x : 0, 
                                y : 0}; }
        
        let mut board :Vec<Figure> = vec![];

        for xy in 0..x*y
        {
            board.push(Figure::null_init());
        }

        Chess{x : x, y : y, board : board}
    }

    pub fn print(&self)
    {
        for y in 0..self.y
        {
            for x in 0..self.x
            {
                print!("{} ", self.board[(y * self.y + x) as usize].figure);
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
            self.board[(y * self.y + x) as usize].figure = "l".to_string();
            self.board[(y * self.y + self.x - x - 1) as usize].figure = "l".to_string();
                        
            self.board[((self.y - y - 1) * self.y + x) as usize].figure = "L".to_string();
            self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = "L".to_string();
                    
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
            self.board[((self.y - y - 1) * self.y + self.x - x - 1) as usize].figure = 
                                                                                    "Q".to_string();
            y = 1 ;
            for x in 0..self.x
            {
                self.board[(y * self.y + x) as usize].figure = "p".to_string(); 
                self.board[((self.y - y - 1) * self.y + x) as usize].figure = "P".to_string(); 
            }
    }
}
