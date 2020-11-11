#version 450 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 instance_offset;
layout(location = 2) in float instance_color;

layout(location = 0) out float vertex_color;

layout(set = 0, binding = 0) uniform Window {
    vec2 window_size;
};

void main() {
    vertex_color = instance_color;
    gl_Position = vec4((position + instance_offset) / window_size, 0.0, 1.0);
}
