mod Collision;
use nalgebra::Vector2;
use ggez::*;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::event::KeyCode;
use nalgebra::*;
use event::*;
use Collision::collision::check_collision;
use rand::Rng;
use ggez::graphics::Rect;



enum ObstacleType
{
    damage,
    heal,
    speed

}


struct Obstacle
{
    
    rect : Rect,
    velocity:Vector2<f32>,
    active:bool,
    respawn_timer:std::time::Duration,
    otype:ObstacleType,
    
}
impl Obstacle{

    pub const fn new(rect:Rect,velocity:Vector2<f32>,otype:ObstacleType) -> Self {
        Obstacle
        {
            rect:rect,
            velocity:velocity,
            active:true,
            respawn_timer:std::time::Duration::from_secs(0),
            otype:otype
        }
    }

}





struct State
{
    delta_time : std::time::Duration,
    text_pos : Point2<f32>,
    health : i32,
    point : i32,
    obs : Vec<Obstacle>,
    spawn_timer : std::time::Duration,
    new_ball_timer : std::time::Duration,
    lose_flag:bool,
    player_speed:f32,
}

impl State
{
    const WIDTH: f32 = 600.0;
    const HEIGHT: f32 = 600.0;
    fn get_player_rect(&self)->Rect{
        Rect::new(self.text_pos.x, self.text_pos.y, 32.0, 32.0)
    }

    fn spawn_new_obstacle(&mut self){
        let ob= generate_obstacle();
        self.obs.push(ob);
    }

    fn get_input_axis(_ctx: & Context)->nalgebra::Vector2<f32>{
        let mut dx:f32=0.0;
        let mut dy:f32=0.0;
        if keyboard::is_key_pressed(_ctx,KeyCode::W){
            dy+=-1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::S){
            dy+=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::D){
            dx+=1.0;
        }
        if keyboard::is_key_pressed(_ctx,KeyCode::A){
            dx+=-1.0;
        }
        nalgebra::Vector2::new(dx,dy)
    }


    fn update_player(&mut self,_ctx: & Context){
        let move_vec = self.player_speed*State::get_input_axis(_ctx)*self.delta_time.as_secs_f32();
        self.text_pos.x+=move_vec.x;
        self.text_pos.y+=move_vec.y;
        if self.text_pos.x<0.0 {
            self.text_pos.x=0.0;
        }
        if self.text_pos.y<0.0 {
            self.text_pos.y=0.0;
        }

        let rightest = State::WIDTH-self.get_player_rect().w;
        let lowest = State::HEIGHT-self.get_player_rect().h;
        if self.text_pos.x>rightest {
            self.text_pos.x=rightest;
        }
        if self.text_pos.y>lowest {
            self.text_pos.y=lowest;
        }
    }


    fn respawn_obstacle(ob:&mut Obstacle)
    {
        let ry = State::HEIGHT;
        let mut rng = rand::thread_rng();
        let r1= rng.gen_range(0.0, 1.0);
        let t =r1*ry;
        ob.rect.y=t;
        ob.rect.x = -10.0;
        ob.active=true;
        
        let mut vrng = rand::thread_rng();
        let r2 = vrng.gen_range(0.0, 1.0);
        ob.velocity = nalgebra::Vector2::new(r2*300.0+200.0,0.0);
        ob.respawn_timer = std::time::Duration::new(0, 0);

        let r3 = vrng.gen_range(0.0, 1.0);
        let otype= if r3<0.8{
            ObstacleType::damage
        }
        else if r3<0.9{
            ObstacleType::heal
        }
        else{
            ObstacleType::speed
        };
        ob.otype = otype;
    }

    fn update_obstacles(&mut self){
        let player_rect =self.get_player_rect();
        for ob in self.obs.iter_mut() {
            ob.rect.x+=ob.velocity.x*self.delta_time.as_secs_f32();
            ob.rect.y+=ob.velocity.y*self.delta_time.as_secs_f32();
            if ob.active{
                if check_collision(&ob.rect,&player_rect){
                    match ob.otype{
                        ObstacleType::damage=>self.health-=1,
                        ObstacleType::heal=>self.health+=1,
                        ObstacleType::speed=>self.player_speed+=20.0,
                    }   

                    ob.active=false;
                    
                }
                if ob.rect.x>State::WIDTH{
                    ob.active=false;
                    self.point+=1;
                }
            }
        }
        let respawn_time = 3.32442;
        let new_ball_time=5.8573;
        self.spawn_timer+=self.delta_time;
        self.new_ball_timer+=self.delta_time;


        for ob in  self.obs.iter_mut(){
            if !ob.active{
                ob.respawn_timer+=self.delta_time;
                if ob.respawn_timer.as_secs_f32()>respawn_time {
                    State::respawn_obstacle(ob);
                }
            }
        }

        self.spawn_timer = std::time::Duration::new(0, 0);
        if self.new_ball_timer.as_secs_f32()>new_ball_time {
            self.spawn_new_obstacle();
            self.new_ball_timer = std::time::Duration::new(0, 0);
        }
    }
}

impl ggez::event::EventHandler for State{
    fn update(&mut self,_ctx: &mut Context)->GameResult<()>{
        self.delta_time =timer::delta(_ctx);

        
        
        if self.health<=0{
            self.lose_flag=true;
        }

        if self.lose_flag {
            
        }
        else
        {
            self.update_player(_ctx);
            self.update_obstacles();
        }

        Ok(())
    }


    fn draw(&mut self,_ctx: &mut Context)->GameResult<()>{
        graphics::clear(_ctx, graphics::BLACK);
        
        let param_ui = graphics::DrawParam::default().dest(Point2::new(400.0,400.0));
        let lose_str= if self.lose_flag{"Game Over!"} else {""};
        let t =Text::new(format!("Health:{}\nPoint:{}\n{}",self.health,self.point,lose_str ));
        t.draw(_ctx, param_ui).ok();

        let param_ui = graphics::DrawParam::default().dest(Point2::new(200.0,200.0));
        let t =Text::new("WASD for move\nDoge red block");
        t.draw(_ctx, param_ui).ok();

        let param_world = graphics::DrawParam::default().dest(Point2::new(0.0,0.0)).color(Color::new(1.0,1.0,1.0,1.0));
        

        let player_rect = self.get_player_rect();
        let player_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(),player_rect , Color::new(1.0,0.0,1.0,1.0))?;
        player_mesh.draw(_ctx,param_world).ok();
        
        for ob in self.obs.iter_mut() {
            if ob.active{
                let draw_color = match ob.otype{
                    ObstacleType::damage => Color::new(1.0,0.0,0.0,1.0),
                    ObstacleType::heal => Color::new(0.0,1.0,0.0,1.0),
                    ObstacleType::speed => Color::new(0.0,0.0,1.0,1.0),
                };
                let mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(),ob.rect ,draw_color)?;
                mesh.draw(_ctx, param_world).ok();

            }
        }
        graphics::present(_ctx).ok();
        Ok(())
    }

    fn mouse_button_down_event(&mut self,_ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32)
    {
        // println!("Press ");

    }
    fn key_down_event(&mut self,_ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool)
    {
        // println!("{:?}", keycode);
    }
}


fn generate_obstacle()->Obstacle
{
    let ry = 600.0;
    let mut rng = rand::thread_rng();
    let r1= rng.gen_range(0.0, 1.0);
    let ty =r1*ry;
    let r = Rect::new(-10.0,ty,40.0,40.0);
    let velo =nalgebra::Vector2::new(200.0,0.0);

    let r2= rng.gen_range(0.0, 1.0);
    let otype= if r2<0.8{
        ObstacleType::damage
    }
    else if r2<0.9{
        ObstacleType::heal
    }
    else{
        ObstacleType::speed
    };
    Obstacle::new(r, velo,otype)
}



fn main() {
    let mut c=conf::Conf::new();
    c.window_mode.height=State::HEIGHT;
    c.window_mode.width=State::WIDTH;
    c.window_setup=c.window_setup.title("Doge");
    let (ref mut ctx,ref mut event_loop)=ContextBuilder::new("hello ggez", "tianxiang").conf(c).build().unwrap();
    

    let mut lov :Vec<Obstacle> = Vec::new();
    let ob =generate_obstacle();
    lov.push(ob);

    let state = &mut State{
        delta_time : std::time::Duration::new(0, 0),
        text_pos:nalgebra::Point2::new(200.0, 200.0),
        health:10,
        point:0,
        obs:lov,
        spawn_timer : std::time::Duration::new(0, 0),
        new_ball_timer : std::time::Duration::new(0,0),
        lose_flag:false,
        player_speed:200.0
    };
    event::run(ctx, event_loop, state).unwrap();


}
