export async function bufferUnordered(data, fn, concurrency = 10) {
	if (data.length === 0) return [];

	// if the concurrency is 0, just run the function on each item
	if (concurrency === 0 || data.length <= concurrency) return Promise.all(data.map(fn));

	let realIndex = 0;

	const results = [];
	const promises = new Map(data.slice(0, concurrency).map((d, i) => [i, fn(d, realIndex++).then(r => [i, r])]));

	// create a promise for each item
	for (const item of data.slice(concurrency)) {
		// if there are more promises than the concurrency, wait for one to finish
		if (promises.size >= concurrency) {
			const [index, result] = await Promise.race(promises.values());

			promises.set(index, fn(item, realIndex++).then(r => [index, r]));
			results.push(result);
		} else {
			// otherwise, add it to the next index in the map
			const index = promises.size;
			promises.set(index, fn(item, realIndex++).then(r => [index, r]));
		}
	}

	// wait for all promises to finish
	results.push(...(await Promise.all(promises.values())).map(([, result]) => result));

	return results;
}