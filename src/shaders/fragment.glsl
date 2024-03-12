#version 130
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec2 texture_size = vec2(textureSize(Texture, 0));
    vec2 texture_cord = uv * texture_size;

    vec2 pixel = floor(texture_cord / 2.0) * 2.0 + vec2(0.5, 0.5);
    vec2 pixel_cord = mod(texture_cord / 2.0, 1.0);

    // Fixes line artifacts on positive edges of image. Likely unnececary. 
    pixel_cord.x += float(uv.x >= 1.0) * (1.0 - pixel_cord.x);
    pixel_cord.y += float(uv.y >= 1.0) * (1.0 - pixel_cord.y);

    pixel.x += float(pixel_cord.y < pixel_cord.x);
    pixel.y += float(pixel_cord.y > 1.0 - pixel_cord.x);

    gl_FragColor = texture2D(Texture, pixel / texture_size);
}
