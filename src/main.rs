use bevy::prelude::*;

const IMAGES: [&str; 5] = [
    "image_64x32.png",
    "image_45x90.png",
    "image_50x150.png",
    "image_150x50.png",
    "image_200x200.png",
];

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let node_width = 100.;
    let node_height = 100.;
    
    let values = [
        Val::Undefined,
        Val::Auto,
        Val::Px(node_width * 0.75),
        Val::Px(node_width),
        Val::Percent(75.),
        Val::Percent(100.),
    ];
    let spacing = 10.;
    let text_height = 20.;
    let mut position = UiRect { left: Val::Px(spacing), top: Val::Px(spacing), ..Default::default() };
    let text_style = TextStyle {
        font: asset_server.load("topaz-8.ttf"),
        font_size: 8.0,
        color: Color::WHITE,
    };
    for &width in &values {
        for &height in &values {
            commands.spawn(TextBundle {
                text: Text::from_section(
                    format!("{:?}\n{:?}", width, height),
                    text_style.clone()
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    position,
                    size: Size { width: Val::Px(node_width), height: Val::Px(text_height) },
                    ..Default::default()
                },
                ..Default::default()
            });
            
            commands.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: position.top.try_add(Val::Px(text_height)).unwrap(),
                        .. position
                    },
                    size: Size { width: Val::Px(node_width), height: Val::Px(node_height) },
                    overflow: Overflow::Hidden,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            })
            .with_children(|builder| {   
                builder
                .spawn(
                    ImageBundle {
                        image: UiImage::new(asset_server.load(IMAGES[0])),    
                        style: Style {
                            size: Size { width, height },
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                );
            });
            position.left = position.left.try_add(Val::Px(node_width + spacing)).unwrap();
        }
        position.left = Val::Px(spacing);
        position.top = position.top.try_add(Val::Px(node_height + text_height + spacing)).unwrap();
    }
}

fn switch_image(
    mut index: Local<usize>,
    input: Res<Input<KeyCode>>,
    assert_server: Res<AssetServer>,
    mut image_query: Query<&mut UiImage>,
) {
    if input.just_pressed(KeyCode::Space) {
        *index = (*index + 1) % IMAGES.len();
        let image_path = IMAGES[*index];
        let image: Handle<Image> = assert_server.load(image_path);
        image_query.for_each_mut(|mut ui_image| {
            *ui_image = UiImage::new(image.clone());
        });
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            window: WindowDescriptor {
                width: 670.,
                height: 790.,
                ..Default::default()
            },
            ..Default::default()
            
        }
    )
    )
    .add_startup_system(spawn)
    .add_system(switch_image)
    .run();
}