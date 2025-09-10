export const transformer = {
  transform(headers, body) {
    headers.push({ key: "x-transformed", value: "true" });
    headers.push({ key: "x-transformer", value: "JavaScript" });
    return [headers, body];
  },
};
