                (glutin::VirtualKeyCode::W, _) => {
                    camera.translate_position(&glm::vec4(0.00, 0.0, 0.01, 0.0));
                }
                (glutin::VirtualKeyCode::S, _) => {
                    camera.translate_position(&glm::vec4(0.00, 0.0, -0.01, 0.0));
                }