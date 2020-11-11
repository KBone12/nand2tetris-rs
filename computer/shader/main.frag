#version 450 core

layout(location = 0) in float vertex_color;
layout(location = 0) out vec4 color;

void main() {
    color = vec4(vertex_color, vertex_color, vertex_color, 1.0);
}
