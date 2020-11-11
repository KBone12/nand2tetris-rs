#version 450 core

layout(location = 0) out vec4 color;

layout(set = 0, binding = 0) uniform Color {
    float white;
};

void main() {
    color = vec4(white, white, white, 1.0);
}
