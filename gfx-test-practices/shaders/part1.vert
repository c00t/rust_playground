// shaders/part1.vert
#version 450 //OpenGL version
#extension GL_ARB_separate_shader_objects : enable

vec2 positions[3] = vec2[](
    vec2(0.0,-0.5),
    vec2(-0.5,0.5),
    vec2(0.5,0.5)
);

void main() {
    vec2 pos = positions[gl_VertexID];
    gl_Position = vec4(pos,0.0,1.0);
}
