#version 330

out vec4 frag_color;
in vec2 fragCoords;

uniform vec2 resolution;
uniform float time;
uniform vec2 camera_position;
uniform float camera_orientation;

float distance_from_sphere(in vec2 point, in vec2 center, float radius)
{
    return length(point - center) - radius;
}

float map_the_world(in vec2 point)
{
    float min_dist = min(distance_from_sphere(point, vec2(-2.0, 0.0), 1.0), distance_from_sphere(point, vec2(2.0, 2.0), 1.0));

    return min_dist;
}

vec3 ray_march(in vec2 ray_origine, in vec2 ray_direction)
{
    float total_distance_traveled = 0.0;
    const int NUMBER_OF_STEPS = 32;
    const float MINIMUM_HIT_DISTANCE = 0.001;
    const float MAXIMUM_TRACE_DISTANCE = 1000.0;
    const vec3 bg_color = vec3(0.0, 0.0, 0.0);

    for (int i = 0; i < NUMBER_OF_STEPS; ++i)
    {
        vec2 current_position = ray_origine + total_distance_traveled * ray_direction;

        float distance_to_closest = map_the_world(current_position);

        if (distance_to_closest < MINIMUM_HIT_DISTANCE)
        {
            return vec3(1.0, 1.0, 1.0) * (0.2/total_distance_traveled);
        }

        if (total_distance_traveled > MAXIMUM_TRACE_DISTANCE)
        {
            break;
        }

        total_distance_traveled += distance_to_closest;
    }

    return bg_color;
}

void main()
{
    vec2 uv = vec2(fragCoords.x * (resolution.x/resolution.y), fragCoords.y);

    vec2 ray_origine = camera_position;
    float x_pos = uv.x/sqrt((uv.x*uv.x)+1);
    float y_pos = sqrt(1-(x_pos*x_pos));
    float x_direction = (x_pos)*cos(camera_orientation)-(y_pos)*sin(camera_orientation);
    float y_direction = (x_pos)*sin(camera_orientation)+(y_pos)*cos(camera_orientation);
    vec2 ray_direction = vec2(x_direction, y_direction);

    vec3 shaded_color = ray_march(ray_origine, ray_direction);

    frag_color = vec4(shaded_color, 1.0);
}