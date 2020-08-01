pub mod Collision{
    pub use ggez::graphics::Rect;
    pub fn check_collision(a: &Rect,b: &Rect)->bool{
        if a.left()>b.right() || a.right()<b.left() || a.bottom()<b.top() || a.top()>b.bottom() 
        {
            false
        }
        else
        {
            true
        }
    }
}

