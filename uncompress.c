#include <stddef.h>
#include <stdio.h>

void uncompress(const char *data, size_t len, char *output) {
	size_t current_pos_in_output = 0;

	for (size_t i = 0; i < len; i++) {
		char byte = data[i];
		int mode = byte >> 7;
		int n = byte & 0b01111111;

		if (mode) {
			for (size_t tmp = 0; tmp < n; tmp++) {
				output[current_pos_in_output++] = data[++i];
			}
		}
		else {
			char next = data[++i];

			for (size_t tmp = 0; tmp < n; tmp++) {
				output[current_pos_in_output++] = next;
			}
		}
	}
}

void undiff(const char *base, char *other, size_t len) {
	for (size_t i = 0; i < len; i++) {
		other[i] = base[i] - other[i];
	}
}

void main() {
	char input[] = { 5, 1 };
	char output[10000] = { 0 };
        uncompress(input, 2, output);

	printf("%d, %d, %d, %d, %d\n", output[0], output[1], output[2], output[3], output[4]);

	char input2[] = {3, 0, 0b10000010 /* 2 */, 1, 0};
        uncompress(input2, 5, output);

	printf("%d, %d, %d, %d, %d\n", output[0], output[1], output[2], output[3], output[4]);
}

