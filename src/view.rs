use crate::browser;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::weapon::gun::Bullet;

pub mod web_renderer {
    use super::*;

    pub struct Renderer {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        cnv: web_sys::HtmlCanvasElement,
        ctx: web_sys::CanvasRenderingContext2d,
    }
    impl Renderer {
        pub fn new(x: f32, y: f32, width: f32, height: f32, cnv_name: &str) -> Self {
            Renderer {
                x,
                y,
                width,
                height,
                cnv: browser::canvas(cnv_name),
                ctx: browser::context(cnv_name),
            }
        }
        pub fn init(&self) {
            let _ = self.cnv.focus();
            self.set_common_style();
        }
        fn set_common_style(&self) {
            self.ctx.set_shadow_color("#d53");
            self.ctx.set_shadow_blur(20.0);
            self.ctx.set_line_join("bevel");
            self.ctx.set_line_width(5.0);
        }
        pub fn clear_canvas(&self) {
            self.ctx.set_fill_style(&"rgb(0,0,0)".into());
            self.ctx
                .clear_rect(self.x.into(), self.y.into(), self.width.into(), self.height.into());
            self.ctx
                .fill_rect(self.x.into(), self.y.into(), self.width.into(), self.height.into());
        }
        pub fn draw_player(&self, player: &Player) {
            self.ctx.set_stroke_style(&"yellow".into()); // &JsValue::from_str("yellow")
            self.ctx.set_fill_style(&"black".into()); // into knows to use JsValue::from_str
            self.ctx.stroke_rect(
                player.x.into(),
                player.y.into(),
                player.width.into(),
                player.height.into(),
            );
            self.ctx.fill_rect(
                player.x.into(),
                player.y.into(),
                player.width.into(),
                player.height.into(),
            );
        }
        pub fn draw_enemy(&self, enemy: &Enemy) {
            self.ctx.set_fill_style(&enemy.color().into());
            if enemy.health > 1 {
                self.ctx.set_stroke_style(&"white".into());
            } else {
                self.ctx.set_stroke_style(&enemy.color().into());
            }

            self.ctx
                .fill_rect(enemy.x.into(), enemy.y.into(), enemy.width.into(), enemy.height.into());
            self.ctx
                .stroke_rect(enemy.x.into(), enemy.y.into(), enemy.width.into(), enemy.height.into());
            //Draw Text
            let health = format!("{}", enemy.health);
            self.ctx.set_fill_style(&"black".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &health,
                (enemy.x + enemy.width / 3.5).into(),
                (enemy.y + enemy.height / 1.5).into(),
            );
        } //^--fn draw_enemy

        //----
        pub fn draw_bullet(&self, bullet: &Bullet) {
            self.ctx.set_stroke_style(&"red".into()); 
            self.ctx.set_fill_style(&"grey".into()); 
            self.ctx.stroke_rect(
                bullet.x.into(),
                bullet.y.into(),
                bullet.width.into(),
                bullet.height.into(),
            );
            self.ctx.fill_rect(
                bullet.x.into(),
                bullet.y.into(),
                bullet.width.into(),
                bullet.height.into(),
            );
        } //^--fn draw_bullet

        pub fn text_score(&self, score: i32) {
            browser::set_span("score", score);
        }
        pub fn text_level(&self, level: i32) {
            browser::set_span("level", level);
        }

        pub fn draw_level_up(&self, level: i32) {
            let msg = format!("R E A D Y   F O R  L E V E L  {}", level);
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                ((self.width / 2.0) - mid_length).into(),
                (self.height / 2.0).into(),
            );
        } //^--fn draw_level_up

        pub fn draw_top_score(&self, best: i32, score: i32) {
            let msg = format!("--- B E S T  S C O R E  {} ---", best); 
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"20px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                ((self.width / 2.0) - mid_length).into(),
                (self.height / 2.0).into(),
            );
            let msg = format!("    Y O U R  S C O R E  {}", score); 
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"20px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                ((self.width / 2.0) - mid_length).into(),
                (self.height / 2.0 + 25.0).into(),
            );
        } //^--fn draw_level_up

        pub fn draw_ready(&self, level: i32) {
            let msg = format!("R E A D Y   L E V E L  {}", level);
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                ((self.width / 2.0) - mid_length).into(),
                (self.height / 2.0).into(),
            );
        } //^--fn draw_ready

        pub fn draw_gameover(&self) {
            let msg = "G A M E   O V E R";
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                ((self.width / 2.0) - mid_length).into(),
                (self.height / 2.0).into(),
            );

            let msg = "Do you want to play again?"; 
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                (mid_length * 2.5).into(),
                (self.height / 1.4).into(),
            );
            let msg = "[n] view Top score    [y] play again";
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"25px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                (mid_length * 2.5).into(),
                (self.height / 1.2).into(),
            );
        } //^--fn draw_gameover

        pub fn draw_menu(&self) {
            let msg = ">>>  >>  I N V A D E R S  <<  <<<";
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"30px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                (self.width/5.5 - mid_length).into(),
                (self.height / 3.0).into(),
            );

            let msg = "Spacebar to start & to shoot";
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"20px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                (20.5).into(),
                (self.height / 1.4).into(),
            );
            let msg = "arrows to move";
            let mid_length = (msg.len() / 2) as f32;
            self.ctx.set_fill_style(&"white".into());
            self.ctx.set_stroke_style(&"red".into());
            self.ctx.set_font(&"20px Arial");
            let _ = self.ctx.fill_text(
                &msg,
                (20.5).into(),
                (self.height / 1.2).into(),
            );
        } //^--fn draw_menu

    } //^-- impl Renderer

} //^--mod web_renderer
