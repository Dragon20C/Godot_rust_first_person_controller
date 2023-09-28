use std::f32::consts::PI;
use godot::prelude::*;
use godot::engine;
use godot::engine::InputEventMouseMotion;
use godot::engine::InputEvent;
use godot::engine::CharacterBody3D;
use godot::engine::CharacterBody3DVirtual;
use godot::prelude::utilities::sin;
use godot::prelude::utilities::cos;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player3D{
    motion_vec : Vector3,
    air_accel: f32,
    decel_speed : f32,
    mouse_sens: f32,
    speed: f32,
    walk_speed: f32,
    sprint_speed: f32,
    jump_force: f32,
    gravity: f32,
    //headbob variables
    bob_freq: f64, // float = 2.4
    bob_amp: f64, // 0.05
    t_bob: f64, 
    camera_node : Option<Gd<Camera3D>>,
    #[base]
    character: Base<CharacterBody3D>

    
}
#[godot_api]
impl Player3D{
    #[func]
    fn apply_headbob(&mut self, time : f64) -> Vector3{
        let mut pos = Vector3::ZERO;
        let pos_x: f64 = sin(time * self.bob_freq) * self.bob_amp;
        let pos_y: f64 = cos(time * self.bob_freq / 2.0) * self.bob_amp;
	    pos.x = pos_x as f32; 
	    pos.y = pos_y as f32;
	
        pos
    }
}



#[godot_api]
impl CharacterBody3DVirtual for Player3D {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Player Loaded");
        // set variables for the player class
        Self {
            motion_vec:Vector3::ZERO,air_accel:3.0,decel_speed:7.0,
            mouse_sens:0.0025,speed:0.0,walk_speed:5.0,sprint_speed:8.0,
            jump_force:4.8,camera_node:None,character:base,gravity:9.8,
             bob_freq: 1.8, bob_amp: 0.03, t_bob: 0.0}

        
    }
    

    fn ready(&mut self){
        // set the mouse mode to captured
        Input::singleton().set_mouse_mode(engine::input::MouseMode::MOUSE_MODE_CAPTURED);
        // set the cam_node to the camera node in the scene
        let cam_node = self.character.get_node_as::<Camera3D>("Head/Camera3D");
        self.camera_node = Some(cam_node);
        // set an inital speed value
        self.speed = self.walk_speed;

    }
    fn unhandled_input(&mut self,event: Gd<InputEvent>){
        // check only for mouse motion
        if let Some(mouse_motion) = event.try_cast::<InputEventMouseMotion>() {
            // set it to a relative variable - for 3d
            let relative = -mouse_motion.get_relative();

            // get the camera out of the option
            if let Some(cam) = self.camera_node.as_mut(){

                // rotate the camera on the x axis times the mouse sensitivity 
                cam.rotate_x(relative.y * self.mouse_sens);

                // Clamp the rotation by PI/2 and set it to the camera
                let rot = Vector3::new(cam.get_rotation().x.clamp(-PI/2.0,PI/2.0),0.0,0.0);
                cam.set_rotation(rot);
                self.character.rotate_y(relative.x * self.mouse_sens);
            }            
        }
    }
    fn physics_process(&mut self, delta: f64){
        // if we are not on the floor we apply gravity
        if !self.character.is_on_floor(){
            self.motion_vec.y -= self.gravity * real::from_f64(delta)
        }

        // get the input singleton into the process
        let input = Input::singleton();

        // if ui_cancel is pressed we exit the game
        if input.is_action_just_pressed("ui_cancel".into()){
            godot_print!("Game Exited.");
            self.character.get_tree().unwrap().quit();
        }

        // if the player is on floor and jump is pressed we apply a jump force to the motion_vec.y
        if self.character.is_on_floor() && input.is_action_just_pressed("jump".into()){
            self.motion_vec.y = self.jump_force
        }

        // if holding the sprint button speed is set to sprint_speed else walk speed
        if input.is_action_pressed("sprint".into()){
            self.speed = self.sprint_speed
        } else {
            self.speed = self.walk_speed
        }

        // we get the player direction based on user input
        let inpur_dir = input.get_vector(
        "left".into(), "right".into(),
        "forward".into(), "backward".into()
        );

        // we get the transform.basis * our input_dir to get the direction normalized.
        let direction = (self.character.get_transform().basis * Vector3::new(inpur_dir.x,0.0,inpur_dir.y)).normalized();
        
        // if the player is on floor and we are moving we apply speed else we lerp to 0.
        if self.character.is_on_floor(){
            if direction != Vector3::ZERO{
                self.motion_vec.x = direction.x * self.speed;
                self.motion_vec.z = direction.z * self.speed;
            } else {
                self.motion_vec.x = self.motion_vec.x.lerp(direction.x * self.speed, real::from_f64(delta) * self.decel_speed);
                self.motion_vec.z = self.motion_vec.z.lerp(direction.z * self.speed, real::from_f64(delta) * self.decel_speed);

                
            }
        } else {
            // if we are not on the floor we decrease the whight value for air acceleration
            self.motion_vec.x = self.motion_vec.x.lerp(direction.x * self.speed, real::from_f64(delta) * self.air_accel);
            self.motion_vec.z = self.motion_vec.z.lerp(direction.z * self.speed, real::from_f64(delta) * self.air_accel);
        }

        // Head bob
        self.t_bob += delta * self.motion_vec.length().as_f64() * if self.character.is_on_floor() {1.0} else {0.0};
        let mut transform = self.camera_node.as_ref().unwrap().get_transform();
        transform.origin = self.apply_headbob(self.t_bob);
        self.camera_node.as_mut().unwrap().set_transform(transform);
        
        //self.camera_node.unwrap().set_transform();
	    //t_bob += delta * velocity.length() * float(is_on_floor())
	    //camera.transform.origin = _headbob(t_bob)

        // then we simply apply it to the characters velocity and enable move and slide.
        self.character.set_velocity(self.motion_vec);
        self.character.move_and_slide();
    }
}