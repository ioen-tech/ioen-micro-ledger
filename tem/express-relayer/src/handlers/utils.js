function deepAddTimestamp(entry, createdAt) {
  for (const key of Object.keys(entry)) {
    if (entry[key] === undefined || entry[key] === null) {
    } else if (Array.isArray(entry[key])) {
      entry[key] = entry[key].map((element) =>
        deepAddTimestamp(element, createdAt)
      );
    } else if (typeof entry[key] === "object") {
      entry[key] = deepAddTimestamp(entry[key], createdAt);
    }
  }

  return {
    ...entry,
    LogTimestamp: createdAt,
  };
}

/**
 * Gets an array of the form [[entryString, [secs, nanosecs]]] and returns it in the form [{...entry, LogTimestamp: MillisTimestamp}]
 * The createdAt property is added to every nested object of the entry
 */
function parseTimestampedEntries(array) {
  return array.map((element) => {
    const entry = element[0];
    const createdAt = Math.floor(
      element[1][0] * 1000 + element[1][1] / 1000000
    );

    return { ...entry, LogTimestamp: createdAt };
  });
}

/**
 * Converts a millis timestamp generated in the UI to a Timestamp in the form that holochain expects
 */
function millisToTimestamp(millisString) {
  if (!millisString || millisString === "undefined") return undefined;

  const millis = parseInt(millisString);

  if (Number.isNaN(millis)) return undefined;

  const secs = Math.floor(millis / 1000);
  const nanos = (millis % 1000) * 1000;

  return [secs, nanos];
}

module.exports = {
  parseTimestampedEntries,
  millisToTimestamp,
};
