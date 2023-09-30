use godot::{prelude::*, engine::{CharacterBody3D, AudioStream, Marker3D}};
use rand::Rng;

#[derive(GodotClass)]
#[class(base=Node)]
struct FootstepPlayer{
    #[export]
    footstep_sounds: Array<Gd<AudioStream>>,//Array<Gd<AudioStreamOggVorbis>>,
    #[export]
    ground_position : Option<Gd<Marker3D>>,
    #[export]
    player_node : Option<Gd<CharacterBody3D>>,
    #[base]
    base: Base<Node>
}

#[godot_api]
impl FootstepPlayer{

    #[func]
    fn play_step(&mut self){
        let mut audio_player = AudioStreamPlayer::new_alloc();
        let mut rng = rand::thread_rng();
        
        let random_index = rng.gen_range(0..=self.footstep_sounds.len() - 1);
        let sound = self.footstep_sounds.get(random_index);
        audio_player.set_stream(sound);
        audio_player.set_pitch_scale(rng.gen_range(0.8..=1.2));

        if let Some(ground) = &mut self.ground_position{
            audio_player.set_autoplay(true);
            ground.add_child(audio_player.upcast());
            //audio_player.connect(signal, callable)
        }
    }
}

#[godot_api]
impl NodeVirtual for FootstepPlayer {
    fn init(base_node: Base<Node>) -> Self {
        godot_print!("Footstep player loaded!");

        Self { footstep_sounds: Array::new(), ground_position: None, player_node: None, base: base_node }
    }

    fn ready(&mut self){
        self.player_node.as_mut().unwrap().connect("stepped".into(), self.base.callable("play_step"));
    }
}
