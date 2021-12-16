#include <stdio.h>
#include <stdint.h>

uint16_t mem_buffer[1024*32];

int
main()
{
	uint16_t *b0 = mem_buffer;
	uint16_t *b1 = mem_buffer + 4096;
	uint16_t *b2 = mem_buffer + 4096 * 2;
	uint16_t *b3 = mem_buffer + 4096 * 3;

	int b0_len = 0;
	int b1_len = 0;
	int b2_len = 0;
	int b3_len = 0;
	
	char cbuff[64];
	while (fgets(cbuff, 64, stdin)) {
		if (cbuff[0] == '\n') break;
			
		b0[b0_len] = 0;
		for (int i = 0; i < 12; i++) {
			b0[b0_len] |= (cbuff[i] == '1') << (11 - i);
		}
		
		b3[b3_len++] = b0[b0_len];
		b0_len++;
	}

	uint16_t most_common = 0;
	uint16_t least_common = 0;

	for (uint16_t mask = (1 << 11); mask > 0; mask >>= 1) {

		for (int i = 0; i < b0_len; i++) {
			if (b0[i] & mask) {
				b1[b1_len++] = b0[i];
			} else {
				b2[b2_len++] = b0[i];
			}
		}

		uint16_t *tmp = b0;
		if (b1_len >= b2_len) {
			b0 = b1;
			b0_len = b1_len;
			b1 = tmp;
		} else {
			b0 = b2;
			b0_len = b2_len;
			b2 = tmp;
		}

		b1_len = 0;
		b2_len = 0;
	}

	most_common = b0[0];
	b0 = b3;
	b0_len = b3_len;

	for (uint16_t mask = (1 << 11); mask > 0; mask >>= 1) {
		
		for (int i = 0; i < b0_len; i++) {
			if (b0[i] & mask) {
				b1[b1_len++] = b0[i];
			} else {
				b2[b2_len++] = b0[i];
			}
		}

		uint16_t *tmp = b0;
		if (b1_len < b2_len) {
			b0 = b1;
			b0_len = b1_len;
			b1 = tmp;
		} else {
			b0 = b2;
			b0_len = b2_len;
			b2 = tmp;
		}

		b1_len = 0;
		b2_len = 0;
	}

	least_common = b0[0];
	printf("%d\n", most_common * least_common);

}
