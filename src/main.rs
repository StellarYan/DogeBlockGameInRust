mod Collision;
use nalgebra::Vector2;
use ggez::*;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::event::KeyCode;
use nalgebra::*;
use event::*;
use Collision::Collision::check_collision;
use rand::Rng;
use ggez::graphics::Rect;




struct Obstacle
{
    rect : Rect,
    velocity:Vector2<f32>,
    active:bool,
}
impl Obstacle{
    pub const fn new(rect:Rect,velocity:Vector2<f32>) -> Self {
        let active=true;
        Obstacle{rect,velocity,active}
    }
    
}





struct State
{
    deltaTime : std::time::Duration,
    text_pos : Point2<f32>,
    health : i32,
    obs : Vec<Obstacle>,
    spawn_timer : std::time::Duration
}

impl State
{
    fn get_player_rect(&self)->Rect{
        Rect::new(self.text_pos.x, self.text_pos.y, 64.0, 64.0)
    }

    fn SpawnNewObstacle(&mut self){
        let ob= generate_obstacle();
        self.obs.push(ob);
    }



    fn GetInputAxis(_ctx: & Context)->nalgebra::Vector2<f32>{
        let mut dx:f32=0.0;
        let mut dy:f32=0.0;
        if keyboard::is_key_pressed(_ctx,KeyCode::W){
            dy=-1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::S){
            dy=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::D){
            dx=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::A){
            dx=-1.0;
        }
        nalgebra::Vector2::new(dx,dy)
    }
}

impl ggez::event::EventHandler for State{





    fn update(&mut self,_ctx: &mut Context)->GameResult<()>{
        self.deltaTime =timer::delta(_ctx);
        let mut dx:f32=0.0;
        let mut dy:f32=0.0;
        if keyboard::is_key_pressed(_ctx,KeyCode::W){
            dy=-1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::S){
            dy=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::D){
            dx=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::A){
            dx=-1.0;
        }
        self.text_pos.x+=dx;
        self.text_pos.y+=dy;


        let player_rect =self.get_player_rect();
        for ob in self.obs.iter_mut() {
            ob.rect.x+=ob.velocity.x;
            ob.rect.y+=ob.velocity.y;
            if ob.active{
                if check_collision(&ob.rect,&player_rect){
                    ob.active=false;
                    self.health-=1;
                }
            }
        }
        let spawn_time = 5.0;
        self.spawn_timer+=self.deltaTime;

        if self.spawn_timer.as_secs_f32()>spawn_time {
            let mut has_ob = false;
            for ob in  self.obs.iter_mut(){
                if !ob.active{
                    let ry = 600.0;
                    let mut rng = rand::thread_rng();
                    let y= rng.gen_range(0.0, 1.0);
                    let t =y*ry;
                    ob.rect.y=0.0;
                    ob.rect.x = t;
                    ob.active=true;
                    has_ob=true;
                }
            }
            if !has_ob
            {
                self.SpawnNewObstacle();
            }
            self.spawn_timer = std::time::Duration::new(0, 0);
        }
        Ok(())
    }


    fn draw(&mut self,_ctx: &mut Context)->GameResult<()>{
        graphics::clear(_ctx, graphics::BLACK);
        
        let param_ui = graphics::DrawParam::default().dest(Point2::new(400.0,400.0));
        let t =Text::new(format!("Health:{}",self.health));
        t.draw(_ctx, param_ui).ok();

        let param_world = graphics::DrawParam::default().dest(Point2::new(0.0,0.0)).color(Color::new(1.0,1.0,1.0,1.0));
        

        let player_rect = self.get_player_rect();
        let player_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(),player_rect , Color::new(1.0,0.0,1.0,1.0))?;
        player_mesh.draw(_ctx,param_world).ok();
        
        for ob in self.obs.iter_mut() {
            if ob.active{
                let mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(),ob.rect , Color::new(1.0,1.0,1.0,1.0))?;
                mesh.draw(_ctx, param_world).ok();

            }
        }
        graphics::present(_ctx).ok();
        Ok(())
    }

    fn mouse_button_down_event(&mut self,_ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32)
    {
        println!("Press ");

    }
    fn key_down_event(&mut self,ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool)
    {
        println!("{:?}", keycode);
    }
}


fn generate_obstacle()->Obstacle
{
    let ry = 600.0;
    let mut rng = rand::thread_rng();
    let y= rng.gen_range(0.0, 1.0);
    let t =y*ry;
    let r = Rect::new(0.0,t,30.0,30.0);
    let velo =nalgebra::Vector2::new(1.0,0.0);
    Obstacle::new(r, velo)
}



fn main() {
    




    let c=conf::Conf::new();
    let (ref mut ctx,ref mut event_loop)=ContextBuilder::new("hello ggez", "tianxiang").conf(c).build().unwrap();


    let mut lov :Vec<Obstacle> = Vec::new();
    let ob =generate_obstacle();
    lov.push(ob);

    let state = &mut State{
        deltaTime : std::time::Duration::new(0, 0),
        text_pos:nalgebra::Point2::new(200.0, 200.0),
        health:10,
        obs:lov,
        spawn_timer : std::time::Duration::new(0, 0),
    };
    event::run(ctx, event_loop, state).unwrap();


}
