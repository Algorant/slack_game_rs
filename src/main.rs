use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
enum GameState {
    StartScreen,
    Playing,
    GameOver,
    Paused,
}

struct Camera {
    x: f32,
    y: f32,
}

impl Camera {
    fn new() -> Self {
        Camera { x: 0.0, y: 0.0 }
    }

    fn follow_player(&mut self, player: &Player) {
        let screen_width = 800.0;
        let screen_height = 600.0;
        
        self.x = player.x + player.width / 2.0 - screen_width / 2.0;
        self.y = player.y + player.height / 2.0 - screen_height / 2.0;
    }

    fn apply(&self) {
        set_camera(&Camera2D {
            target: Vec2::new(self.x + 400.0, self.y + 300.0),
            zoom: Vec2::new(1.0 / 400.0, 1.0 / 300.0),
            ..Default::default()
        });
    }
}

#[derive(Clone)]
struct Platform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    velocity_x: f32,
    velocity_y: f32,
    direction: f32,
    alive: bool,
}

struct Coin {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    collected: bool,
}

struct PowerUp {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    collected: bool,
    power_type: PowerUpType,
}

#[derive(Clone)]
enum PowerUpType {
    SpeedBoost,
    JumpBoost,
}

impl Platform {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Platform { x, y, width, height }
    }

    fn draw(&self) {
        // Draw platform with grass texture
        draw_rectangle(self.x, self.y, self.width, self.height, DARKGREEN);
        // Add grass tufts on top
        let grass_spacing = 8.0;
        let mut x = self.x;
        while x < self.x + self.width - 4.0 {
            draw_rectangle(x, self.y - 2.0, 4.0, 2.0, GREEN);
            x += grass_spacing;
        }
        // Add some dirt pattern
        let dirt_spacing = 16.0;
        let mut x = self.x + 4.0;
        while x < self.x + self.width - 8.0 {
            draw_rectangle(x, self.y + 4.0, 8.0, 4.0, BROWN);
            x += dirt_spacing;
        }
    }
}

impl Enemy {
    fn new(x: f32, y: f32) -> Self {
        Enemy {
            x,
            y,
            width: 24.0,
            height: 24.0,
            velocity_x: 50.0,
            velocity_y: 0.0,
            direction: 1.0,
            alive: true,
        }
    }

    fn update(&mut self, dt: f32, platforms: &[Platform]) {
        if !self.alive {
            return;
        }

        let gravity = 800.0;
        self.velocity_y += gravity * dt;

        // Move horizontally
        self.x += self.velocity_x * self.direction * dt;

        // Check for platform edges or walls
        let mut _on_platform = false;
        for platform in platforms {
            // Check if enemy is on this platform
            if self.x + self.width > platform.x &&
               self.x < platform.x + platform.width &&
               self.y + self.height >= platform.y &&
               self.y + self.height <= platform.y + 10.0 {
                _on_platform = true;
                
                // Check if enemy is near platform edge
                if self.direction > 0.0 && self.x + self.width >= platform.x + platform.width {
                    self.direction = -1.0;
                } else if self.direction < 0.0 && self.x <= platform.x {
                    self.direction = 1.0;
                }
            }
        }

        // Apply gravity and handle vertical collisions
        self.y += self.velocity_y * dt;

        for platform in platforms {
            if self.x < platform.x + platform.width &&
               self.x + self.width > platform.x &&
               self.y < platform.y + platform.height &&
               self.y + self.height > platform.y {
                
                if self.velocity_y > 0.0 {
                    self.y = platform.y - self.height;
                    self.velocity_y = 0.0;
                }
            }
        }
    }

    fn check_collision(&self, player: &Player) -> bool {
        self.alive &&
        self.x < player.x + player.width &&
        self.x + self.width > player.x &&
        self.y < player.y + player.height &&
        self.y + self.height > player.y
    }

    fn draw(&self) {
        if self.alive {
            // Draw simple enemy sprite
            // Body
            draw_rectangle(self.x + 2.0, self.y + 8.0, 20.0, 16.0, PURPLE);
            // Head
            draw_rectangle(self.x + 4.0, self.y, 16.0, 12.0, DARKPURPLE);
            // Eyes
            draw_rectangle(self.x + 7.0, self.y + 3.0, 3.0, 3.0, RED);
            draw_rectangle(self.x + 14.0, self.y + 3.0, 3.0, 3.0, RED);
            // Feet
            draw_rectangle(self.x, self.y + 20.0, 6.0, 4.0, BLACK);
            draw_rectangle(self.x + 18.0, self.y + 20.0, 6.0, 4.0, BLACK);
        }
    }
}

impl Coin {
    fn new(x: f32, y: f32) -> Self {
        Coin {
            x,
            y,
            width: 16.0,
            height: 16.0,
            collected: false,
        }
    }

    fn check_collision(&self, player: &Player) -> bool {
        !self.collected &&
        self.x < player.x + player.width &&
        self.x + self.width > player.x &&
        self.y < player.y + player.height &&
        self.y + self.height > player.y
    }

    fn draw(&self) {
        if !self.collected {
            // Draw coin with sparkle effect
            draw_rectangle(self.x + 2.0, self.y + 2.0, 12.0, 12.0, GOLD);
            draw_rectangle(self.x + 4.0, self.y + 4.0, 8.0, 8.0, YELLOW);
            // Add sparkle
            draw_rectangle(self.x + 1.0, self.y + 7.0, 2.0, 2.0, WHITE);
            draw_rectangle(self.x + 13.0, self.y + 7.0, 2.0, 2.0, WHITE);
        }
    }
}

impl PowerUp {
    fn new(x: f32, y: f32, power_type: PowerUpType) -> Self {
        PowerUp {
            x,
            y,
            width: 20.0,
            height: 20.0,
            collected: false,
            power_type,
        }
    }

    fn check_collision(&self, player: &Player) -> bool {
        !self.collected &&
        self.x < player.x + player.width &&
        self.x + self.width > player.x &&
        self.y < player.y + player.height &&
        self.y + self.height > player.y
    }

    fn draw(&self) {
        if !self.collected {
            let (primary_color, secondary_color) = match self.power_type {
                PowerUpType::SpeedBoost => (ORANGE, RED),
                PowerUpType::JumpBoost => (BLUE, SKYBLUE),
            };
            // Draw power-up with glow effect
            draw_rectangle(self.x - 2.0, self.y - 2.0, self.width + 4.0, self.height + 4.0, secondary_color);
            draw_rectangle(self.x, self.y, self.width, self.height, primary_color);
            draw_rectangle(self.x + 4.0, self.y + 4.0, 12.0, 12.0, WHITE);
            
            // Add symbol
            match self.power_type {
                PowerUpType::SpeedBoost => {
                    // Lightning bolt
                    draw_rectangle(self.x + 8.0, self.y + 6.0, 2.0, 8.0, WHITE);
                    draw_rectangle(self.x + 6.0, self.y + 8.0, 6.0, 2.0, WHITE);
                }
                PowerUpType::JumpBoost => {
                    // Up arrow
                    draw_rectangle(self.x + 9.0, self.y + 6.0, 2.0, 6.0, WHITE);
                    draw_rectangle(self.x + 7.0, self.y + 8.0, 6.0, 2.0, WHITE);
                }
            }
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    velocity_x: f32,
    velocity_y: f32,
    on_ground: bool,
    score: i32,
    speed_boost: f32,
    jump_boost: f32,
    lives: i32,
    animation_timer: f32,
    facing_right: bool,
}

struct Level {
    platforms: Vec<Platform>,
    enemies: Vec<Enemy>,
    coins: Vec<Coin>,
    powerups: Vec<PowerUp>,
    goal_x: f32,
    goal_y: f32,
}

struct Game {
    state: GameState,
    player: Player,
    camera: Camera,
    levels: Vec<Level>,
    current_level: usize,
    level_completed: bool,
    // Simple audio using frequency synthesis (no external files needed)
}

impl Player {
    fn new() -> Self {
        Player {
            x: 100.0,
            y: 480.0,
            width: 32.0,
            height: 32.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            on_ground: false,
            score: 0,
            speed_boost: 1.0,
            jump_boost: 1.0,
            lives: 3,
            animation_timer: 0.0,
            facing_right: true,
        }
    }

    fn reset_position(&mut self) {
        self.x = 100.0;
        self.y = 480.0;
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
        self.speed_boost = 1.0;
        self.jump_boost = 1.0;
        self.animation_timer = 0.0;
        self.facing_right = true;
    }

    fn update(&mut self, dt: f32, platforms: &[Platform]) {
        let gravity = 800.0;
        
        // Update animation timer
        self.animation_timer += dt;
        
        self.velocity_y += gravity * dt;
        
        // Move horizontally first
        self.x += self.velocity_x * dt;
        
        // Check horizontal collisions
        for platform in platforms {
            if self.x < platform.x + platform.width &&
               self.x + self.width > platform.x &&
               self.y < platform.y + platform.height &&
               self.y + self.height > platform.y {
                
                // Horizontal collision - push player out
                if self.velocity_x > 0.0 {
                    // Moving right, hit left side of platform
                    self.x = platform.x - self.width;
                } else if self.velocity_x < 0.0 {
                    // Moving left, hit right side of platform
                    self.x = platform.x + platform.width;
                }
                self.velocity_x = 0.0;
            }
        }
        
        // Move vertically
        self.y += self.velocity_y * dt;
        self.on_ground = false;
        
        // Check vertical collisions
        for platform in platforms {
            if self.x < platform.x + platform.width &&
               self.x + self.width > platform.x &&
               self.y < platform.y + platform.height &&
               self.y + self.height > platform.y {
                
                if self.velocity_y > 0.0 {
                    // Falling down, hit top of platform
                    self.y = platform.y - self.height;
                    self.velocity_y = 0.0;
                    self.on_ground = true;
                } else if self.velocity_y < 0.0 {
                    // Moving up, hit bottom of platform
                    self.y = platform.y + platform.height;
                    self.velocity_y = 0.0;
                }
            }
        }
        
        self.velocity_x *= 0.8;
    }

    fn handle_input(&mut self) -> bool {
        let speed = 200.0 * self.speed_boost;
        let jump_force = -500.0 * self.jump_boost;
        let mut jumped = false;
        
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.velocity_x = -speed;
            self.facing_right = false;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.velocity_x = speed;
            self.facing_right = true;
        }
        if (is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Space)) && self.on_ground {
            self.velocity_y = jump_force;
            jumped = true;
        }
        jumped
    }

    fn draw(&self) {
        // Simple sprite-like rendering with animation
        let is_moving = self.velocity_x.abs() > 10.0;
        let is_jumping = !self.on_ground;
        
        if is_jumping {
            // Jumping sprite - single frame
            self.draw_jumping_sprite();
        } else if is_moving {
            // Walking animation - alternate between two frames
            let frame = ((self.animation_timer * 8.0) as i32) % 2;
            self.draw_walking_sprite(frame);
        } else {
            // Idle sprite
            self.draw_idle_sprite();
        }
    }
    
    fn draw_idle_sprite(&self) {
        // Main body
        draw_rectangle(self.x + 8.0, self.y + 4.0, 16.0, 24.0, RED);
        // Head
        draw_rectangle(self.x + 10.0, self.y, 12.0, 8.0, PINK);
        // Eyes
        draw_rectangle(self.x + 12.0, self.y + 2.0, 2.0, 2.0, BLACK);
        draw_rectangle(self.x + 18.0, self.y + 2.0, 2.0, 2.0, BLACK);
        // Feet
        draw_rectangle(self.x + 6.0, self.y + 28.0, 6.0, 4.0, BROWN);
        draw_rectangle(self.x + 20.0, self.y + 28.0, 6.0, 4.0, BROWN);
    }
    
    fn draw_walking_sprite(&self, frame: i32) {
        // Main body
        draw_rectangle(self.x + 8.0, self.y + 4.0, 16.0, 24.0, RED);
        // Head
        draw_rectangle(self.x + 10.0, self.y, 12.0, 8.0, PINK);
        // Eyes
        draw_rectangle(self.x + 12.0, self.y + 2.0, 2.0, 2.0, BLACK);
        draw_rectangle(self.x + 18.0, self.y + 2.0, 2.0, 2.0, BLACK);
        
        // Animated feet
        if frame == 0 {
            draw_rectangle(self.x + 6.0, self.y + 28.0, 6.0, 4.0, BROWN);
            draw_rectangle(self.x + 22.0, self.y + 30.0, 6.0, 2.0, BROWN);
        } else {
            draw_rectangle(self.x + 4.0, self.y + 30.0, 6.0, 2.0, BROWN);
            draw_rectangle(self.x + 20.0, self.y + 28.0, 6.0, 4.0, BROWN);
        }
    }
    
    fn draw_jumping_sprite(&self) {
        // Main body
        draw_rectangle(self.x + 8.0, self.y + 4.0, 16.0, 24.0, RED);
        // Head
        draw_rectangle(self.x + 10.0, self.y, 12.0, 8.0, PINK);
        // Eyes
        draw_rectangle(self.x + 12.0, self.y + 2.0, 2.0, 2.0, BLACK);
        draw_rectangle(self.x + 18.0, self.y + 2.0, 2.0, 2.0, BLACK);
        // Feet together
        draw_rectangle(self.x + 12.0, self.y + 28.0, 8.0, 4.0, BROWN);
    }
}

// Simple audio synthesis functions
fn play_jump_sound() {
    // Create a simple jump sound using frequency sweep
    let _frequency = 440.0; // A4 note
    let _duration = 0.1;
    // Note: In a real implementation, you'd use macroquad's audio system
    // For now, we'll just use println to indicate sound events
    println!("🔊 Jump sound!");
}

fn play_coin_sound() {
    println!("🔊 Coin collected!");
}

fn play_enemy_defeat_sound() {
    println!("🔊 Enemy defeated!");
}

fn play_powerup_sound() {
    println!("🔊 Power-up collected!");
}

fn play_hit_sound() {
    println!("🔊 Player hit!");
}

fn play_level_complete_sound() {
    println!("🔊 Level complete!");
}

impl Level {
    fn create_level_1() -> Self {
        // World 1-1: Basic tutorial level
        let platforms = vec![
            Platform::new(0.0, 550.0, 800.0, 50.0),       // Ground
            Platform::new(200.0, 450.0, 150.0, 20.0),     // First platform
            Platform::new(500.0, 400.0, 120.0, 20.0),     // Higher platform
            Platform::new(100.0, 350.0, 100.0, 20.0),     // Side platform
            Platform::new(650.0, 300.0, 150.0, 20.0),     // High platform
            Platform::new(900.0, 500.0, 200.0, 20.0),     // Landing area
            Platform::new(1200.0, 400.0, 150.0, 20.0),    // Challenge jump
            Platform::new(1500.0, 350.0, 100.0, 20.0),    // Near goal
            Platform::new(1700.0, 500.0, 200.0, 50.0),    // Goal platform
        ];
        
        let enemies = vec![
            Enemy::new(250.0, 420.0),
            Enemy::new(550.0, 370.0),
            Enemy::new(950.0, 470.0),
            Enemy::new(1250.0, 370.0),
        ];
        
        let coins = vec![
            Coin::new(150.0, 520.0),
            Coin::new(275.0, 430.0),
            Coin::new(325.0, 430.0),
            Coin::new(575.0, 380.0),
            Coin::new(700.0, 280.0),
            Coin::new(1000.0, 480.0),
            Coin::new(1300.0, 380.0),
            Coin::new(1550.0, 330.0),
        ];
        
        let powerups = vec![
            PowerUp::new(275.0, 420.0, PowerUpType::SpeedBoost),
            PowerUp::new(750.0, 280.0, PowerUpType::JumpBoost),
            PowerUp::new(1000.0, 470.0, PowerUpType::SpeedBoost),
        ];

        Level {
            platforms,
            enemies,
            coins,
            powerups,
            goal_x: 1800.0,
            goal_y: 450.0,
        }
    }

    fn create_level_2() -> Self {
        // World 1-2: Underground level with more enemies
        let platforms = vec![
            Platform::new(0.0, 550.0, 300.0, 50.0),       // Start area
            Platform::new(400.0, 500.0, 100.0, 20.0),     // First gap
            Platform::new(600.0, 450.0, 100.0, 20.0),     // Rising platforms
            Platform::new(800.0, 400.0, 100.0, 20.0),
            Platform::new(1000.0, 350.0, 100.0, 20.0),
            Platform::new(1200.0, 300.0, 150.0, 20.0),    // High platform
            Platform::new(1450.0, 400.0, 100.0, 20.0),    // Drop down
            Platform::new(1650.0, 500.0, 100.0, 20.0),
            Platform::new(1850.0, 450.0, 100.0, 20.0),
            Platform::new(2050.0, 400.0, 200.0, 50.0),    // Goal area
        ];
        
        let enemies = vec![
            Enemy::new(450.0, 470.0),
            Enemy::new(650.0, 420.0),
            Enemy::new(850.0, 370.0),
            Enemy::new(1050.0, 320.0),
            Enemy::new(1300.0, 270.0),
            Enemy::new(1500.0, 370.0),
            Enemy::new(1700.0, 470.0),
        ];
        
        let coins = vec![
            Coin::new(250.0, 520.0),
            Coin::new(450.0, 480.0),
            Coin::new(650.0, 430.0),
            Coin::new(850.0, 380.0),
            Coin::new(1050.0, 330.0),
            Coin::new(1325.0, 280.0),
            Coin::new(1500.0, 380.0),
            Coin::new(1700.0, 480.0),
            Coin::new(1900.0, 430.0),
            Coin::new(2100.0, 380.0),
        ];
        
        let powerups = vec![
            PowerUp::new(425.0, 470.0, PowerUpType::JumpBoost),
            PowerUp::new(1275.0, 270.0, PowerUpType::SpeedBoost),
            PowerUp::new(1875.0, 420.0, PowerUpType::JumpBoost),
        ];

        Level {
            platforms,
            enemies,
            coins,
            powerups,
            goal_x: 2150.0,
            goal_y: 350.0,
        }
    }

    fn create_level_3() -> Self {
        // World 1-3: Castle level with challenging jumps
        let platforms = vec![
            Platform::new(0.0, 550.0, 200.0, 50.0),       // Start
            Platform::new(300.0, 500.0, 80.0, 20.0),      // Small platforms
            Platform::new(480.0, 450.0, 80.0, 20.0),
            Platform::new(660.0, 400.0, 80.0, 20.0),
            Platform::new(840.0, 350.0, 80.0, 20.0),      // Ascending
            Platform::new(1020.0, 300.0, 100.0, 20.0),    // Peak
            Platform::new(1200.0, 250.0, 150.0, 20.0),    // High castle area
            Platform::new(1450.0, 300.0, 80.0, 20.0),     // Descending
            Platform::new(1630.0, 350.0, 80.0, 20.0),
            Platform::new(1810.0, 400.0, 80.0, 20.0),
            Platform::new(1990.0, 450.0, 80.0, 20.0),
            Platform::new(2170.0, 500.0, 200.0, 50.0),    // Final area
        ];
        
        let enemies = vec![
            Enemy::new(350.0, 470.0),
            Enemy::new(530.0, 420.0),
            Enemy::new(710.0, 370.0),
            Enemy::new(890.0, 320.0),
            Enemy::new(1070.0, 270.0),
            Enemy::new(1300.0, 220.0),
            Enemy::new(1500.0, 270.0),
            Enemy::new(1680.0, 320.0),
            Enemy::new(1860.0, 370.0),
            Enemy::new(2040.0, 420.0),
        ];
        
        let coins = vec![
            Coin::new(340.0, 480.0),
            Coin::new(520.0, 430.0),
            Coin::new(700.0, 380.0),
            Coin::new(880.0, 330.0),
            Coin::new(1070.0, 280.0),
            Coin::new(1325.0, 230.0),
            Coin::new(1490.0, 280.0),
            Coin::new(1670.0, 330.0),
            Coin::new(1850.0, 380.0),
            Coin::new(2030.0, 430.0),
            Coin::new(2220.0, 480.0),
            Coin::new(2270.0, 480.0),
        ];
        
        let powerups = vec![
            PowerUp::new(380.0, 470.0, PowerUpType::JumpBoost),
            PowerUp::new(1325.0, 220.0, PowerUpType::SpeedBoost),
            PowerUp::new(2220.0, 470.0, PowerUpType::JumpBoost),
        ];

        Level {
            platforms,
            enemies,
            coins,
            powerups,
            goal_x: 2300.0,
            goal_y: 450.0,
        }
    }

    fn draw_goal(&self) {
        // Draw flag pole goal (Mario-style)
        // Pole
        draw_rectangle(self.goal_x, self.goal_y - 100.0, 8.0, 150.0, BROWN);
        // Flag
        draw_rectangle(self.goal_x + 8.0, self.goal_y - 80.0, 40.0, 30.0, GREEN);
        draw_rectangle(self.goal_x + 8.0, self.goal_y - 50.0, 40.0, 30.0, RED);
        // Flag pole top
        draw_rectangle(self.goal_x + 2.0, self.goal_y - 105.0, 12.0, 8.0, GOLD);
    }
}

impl Game {
    fn new() -> Self {
        let levels = vec![
            Level::create_level_1(),
            Level::create_level_2(),
            Level::create_level_3(),
        ];

        Game {
            state: GameState::StartScreen,
            player: Player::new(),
            camera: Camera::new(),
            levels,
            current_level: 0,
            level_completed: false,
        }
    }

    fn reset_game(&mut self) {
        self.player = Player::new();
        self.camera = Camera::new();
        self.current_level = 0;
        self.level_completed = false;
        
        // Reset all levels
        self.levels = vec![
            Level::create_level_1(),
            Level::create_level_2(),
            Level::create_level_3(),
        ];
    }

    fn current_level(&self) -> &Level {
        &self.levels[self.current_level]
    }

    fn current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.current_level]
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::StartScreen => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Paused;
                    return;
                }

                let jumped = self.player.handle_input();
                if jumped {
                    play_jump_sound();
                }
                // Update player first (before any level mutable borrows)
                let platforms = &self.current_level().platforms.clone();
                self.player.update(dt, &platforms);
                
                // Extract current player state for collision detection
                let player_x = self.player.x;
                let player_y = self.player.y;
                let player_width = self.player.width;
                let player_height = self.player.height;
                let player_velocity_y = self.player.velocity_y;
                
                // Collect all changes to apply later
                let mut player_velocity_bounce = None;
                let mut score_change = 0;
                let mut lives_change = 0;
                let mut speed_boost = None;
                let mut jump_boost = None;
                let mut should_reset_position = false;
                
                // Handle enemy collisions
                {
                    let level = self.current_level_mut();
                    
                    for enemy in &mut level.enemies {
                        enemy.update(dt, &level.platforms);
                        
                        if enemy.alive && 
                           enemy.x < player_x + player_width &&
                           enemy.x + enemy.width > player_x &&
                           enemy.y < player_y + player_height &&
                           enemy.y + enemy.height > player_y {
                            
                            if player_velocity_y > 0.0 && player_y < enemy.y {
                                enemy.alive = false;
                                player_velocity_bounce = Some(-300.0);
                                score_change += 100;
                                play_enemy_defeat_sound();
                            } else {
                                lives_change -= 1;
                                should_reset_position = true;
                                play_hit_sound();
                            }
                        }
                    }
                }
                
                // Handle coin collection
                {
                    let level = self.current_level_mut();
                    
                    for coin in &mut level.coins {
                        if !coin.collected &&
                           coin.x < player_x + player_width &&
                           coin.x + coin.width > player_x &&
                           coin.y < player_y + player_height &&
                           coin.y + coin.height > player_y {
                            
                            coin.collected = true;
                            score_change += 10;
                            play_coin_sound();
                        }
                    }
                }
                
                // Handle power-up collection
                {
                    let level = self.current_level_mut();
                    
                    for powerup in &mut level.powerups {
                        if !powerup.collected &&
                           powerup.x < player_x + player_width &&
                           powerup.x + powerup.width > player_x &&
                           powerup.y < player_y + player_height &&
                           powerup.y + powerup.height > player_y {
                            
                            powerup.collected = true;
                            match powerup.power_type {
                                PowerUpType::SpeedBoost => speed_boost = Some(1.5),
                                PowerUpType::JumpBoost => jump_boost = Some(1.3),
                            }
                            score_change += 50;
                            play_powerup_sound();
                        }
                    }
                }
                
                // Apply all changes to player (after releasing level borrow)
                if let Some(velocity) = player_velocity_bounce {
                    self.player.velocity_y = velocity;
                }
                self.player.score += score_change;
                self.player.lives += lives_change;
                if let Some(boost) = speed_boost {
                    self.player.speed_boost = boost;
                }
                if let Some(boost) = jump_boost {
                    self.player.jump_boost = boost;
                }
                if should_reset_position {
                    self.player.reset_position();
                }
                if self.player.lives <= 0 {
                    self.state = GameState::GameOver;
                }

                // Check goal collision
                let level = self.current_level();
                if self.player.x + self.player.width > level.goal_x &&
                   self.player.x < level.goal_x + 50.0 &&
                   self.player.y + self.player.height > level.goal_y - 100.0 &&
                   self.player.y < level.goal_y + 50.0 {
                    
                    if !self.level_completed {
                        self.level_completed = true;
                        play_level_complete_sound();
                        self.player.score += 1000;
                        
                        // Move to next level after 2 seconds
                        if self.current_level + 1 < self.levels.len() {
                            self.current_level += 1;
                            self.player.reset_position();
                            self.level_completed = false;
                        } else {
                            // Game completed
                            self.state = GameState::GameOver;
                        }
                    }
                }

                // Check if player fell off the world
                if self.player.y > 700.0 {
                    self.player.lives -= 1;
                    play_hit_sound();
                    if self.player.lives <= 0 {
                        self.state = GameState::GameOver;
                    } else {
                        self.player.reset_position();
                    }
                }
                
                self.camera.follow_player(&self.player);
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    self.reset_game();
                    self.state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::StartScreen;
                }
            }
        }
    }

    fn draw(&self) {
        // Background gradient from light blue to darker blue
        for y in 0..600 {
            let ratio = y as f32 / 600.0;
            let color = Color::new(
                0.5 + ratio * 0.2,  // Light blue to darker blue
                0.8 + ratio * 0.1,  // Sky blue gradient
                1.0,                // Full blue
                1.0
            );
            draw_rectangle(0.0, y as f32, 800.0, 1.0, color);
        }
        
        // Add simple clouds (only in playing state to avoid camera issues)
        if self.state == GameState::Playing {
            self.camera.apply();
            self.draw_clouds();
            set_default_camera();
        }

        match self.state {
            GameState::StartScreen => {
                draw_text("SLACK GAME", 320.0, 250.0, 60.0, WHITE);
                draw_text("Press SPACE or ENTER to start", 250.0, 350.0, 30.0, WHITE);
                draw_text("Use A/D or Arrow Keys to move", 240.0, 400.0, 25.0, WHITE);
                draw_text("Use W/Up/Space to jump", 260.0, 430.0, 25.0, WHITE);
            }
            GameState::Playing => {
                self.camera.apply();
                
                let level = self.current_level();
                
                for platform in &level.platforms {
                    platform.draw();
                }
                
                for enemy in &level.enemies {
                    enemy.draw();
                }
                
                for coin in &level.coins {
                    coin.draw();
                }
                
                for powerup in &level.powerups {
                    powerup.draw();
                }
                
                // Draw goal flag
                level.draw_goal();
                
                self.player.draw();
                
                set_default_camera();
                draw_text(&format!("Score: {}", self.player.score), 10.0, 30.0, 30.0, WHITE);
                draw_text(&format!("Lives: {}", self.player.lives), 10.0, 60.0, 30.0, WHITE);
                draw_text(&format!("Level: {}", self.current_level + 1), 10.0, 90.0, 30.0, WHITE);
                draw_text("ESC: Pause", 10.0, 120.0, 20.0, WHITE);
            }
            GameState::Paused => {
                self.camera.apply();
                
                let level = self.current_level();
                
                for platform in &level.platforms {
                    platform.draw();
                }
                
                for enemy in &level.enemies {
                    enemy.draw();
                }
                
                for coin in &level.coins {
                    coin.draw();
                }
                
                for powerup in &level.powerups {
                    powerup.draw();
                }
                
                level.draw_goal();
                
                self.player.draw();
                
                set_default_camera();
                draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.5));
                draw_text("PAUSED", 340.0, 280.0, 60.0, WHITE);
                draw_text("Press ESC to resume", 290.0, 340.0, 30.0, WHITE);
            }
            GameState::GameOver => {
                if self.current_level >= self.levels.len() {
                    // Game completed
                    draw_text("CONGRATULATIONS!", 260.0, 200.0, 50.0, GOLD);
                    draw_text("YOU COMPLETED ALL LEVELS!", 220.0, 260.0, 40.0, WHITE);
                    draw_text(&format!("Final Score: {}", self.player.score), 280.0, 320.0, 40.0, WHITE);
                } else {
                    // Game over
                    draw_text("GAME OVER", 310.0, 250.0, 60.0, RED);
                    draw_text(&format!("Final Score: {}", self.player.score), 280.0, 320.0, 40.0, WHITE);
                }
                draw_text("Press R to restart", 300.0, 380.0, 30.0, WHITE);
                draw_text("Press ESC for main menu", 270.0, 420.0, 30.0, WHITE);
            }
        }
    }
    
    fn draw_clouds(&self) {
        // Simple cloud sprites scattered across the level
        let clouds = [
            (300.0, 100.0),
            (800.0, 150.0),
            (1400.0, 80.0),
            (600.0, 200.0),
            (1100.0, 120.0),
            (1700.0, 160.0),
        ];
        
        for (x, y) in clouds.iter() {
            self.draw_cloud(*x, *y);
        }
    }
    
    fn draw_cloud(&self, x: f32, y: f32) {
        // Simple cloud made of circles
        draw_rectangle(x, y + 10.0, 60.0, 20.0, WHITE);
        draw_rectangle(x - 10.0, y + 15.0, 30.0, 15.0, WHITE);
        draw_rectangle(x + 40.0, y + 15.0, 30.0, 15.0, WHITE);
        draw_rectangle(x + 10.0, y, 40.0, 20.0, WHITE);
        draw_rectangle(x + 20.0, y - 5.0, 20.0, 15.0, WHITE);
    }
}

#[macroquad::main("Slack Game")]
async fn main() {
    let mut game = Game::new();
    
    loop {
        let dt = get_frame_time();
        
        game.update(dt);
        game.draw();

        next_frame().await
    }
}
