#version 450 core

layout(location = 0) in vec2 position;

layout(set = 0, binding = 0) uniform Window {
    vec2 window_size;
};

void main() {
    gl_Position = vec4(position / window_size, 0.0, 1.0);
}
