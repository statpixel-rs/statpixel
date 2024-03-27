struct Camera {
	viewPos: vec4<f32>,
	viewProj: mat4x4<f32>,
};

@group(1) @binding(0) var<uniform> camera: Camera;

struct Light {
	position: vec3<f32>,
	color: vec3<f32>,
}

@group(2) @binding(0) var<uniform> light: Light;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) texCoords: vec2<f32>,
	@location(2) normal: vec3<f32>,
};

struct InstanceInput {
	@location(5) modelMatrix0: vec4<f32>,
	@location(6) modelMatrix1: vec4<f32>,
	@location(7) modelMatrix2: vec4<f32>,
	@location(8) modelMatrix3: vec4<f32>,
	@location(9) normalMatrix0: vec3<f32>,
	@location(10) normalMatrix1: vec3<f32>,
	@location(11) normalMatrix2: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) clipPosition: vec4<f32>,
	@location(0) texCoords: vec2<f32>,
	@location(1) worldNormal: vec3<f32>,
	@location(2) worldPosition: vec3<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
	let modelMatrix = mat4x4<f32>(
		instance.modelMatrix0, instance.modelMatrix1,
		instance.modelMatrix2, instance.modelMatrix3,
	);

	let normalMatrix = mat3x3<f32>(
		instance.normalMatrix0, instance.normalMatrix1, instance.normalMatrix2,
	);

	var output: VertexOutput;

	output.texCoords = vertex.texCoords;
	output.worldNormal = normalMatrix * vertex.normal;
	output.worldPosition = (modelMatrix * vec4<f32>(vertex.position, 1.0)).xyz;
	output.clipPosition = camera.viewProj * vec4<f32>(output.worldPosition, 1.0);

	return output;
}

@group(0) @binding(0) var tDiffuse: texture_2d<f32>;
@group(0) @binding(1) var sDiffuse: sampler;

@fragment
fn fs_main(fragmentInput: VertexOutput) -> @location(0) vec4<f32> {
	let objectColor = textureSample(tDiffuse, sDiffuse, fragmentInput.texCoords);
	let lightDir = normalize(light.position - fragmentInput.worldPosition);
	let ambient = 0.1 * light.color;
	let diffuse = max(dot(fragmentInput.worldNormal, lightDir), 0.0) * light.color;
	let viewDir = normalize(camera.viewPos.xyz - fragmentInput.worldPosition);
	let halfDir = normalize(viewDir + lightDir);
	let specular = pow(max(dot(fragmentInput.worldNormal, halfDir), 0.0), 32.0) * light.color;
	let color = (ambient + diffuse + specular) * objectColor.xyz;

	return vec4<f32>(color, objectColor.a);
}
