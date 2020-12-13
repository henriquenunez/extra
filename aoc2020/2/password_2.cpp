#include <cstring>
#include <cstdio>
#include <cstdlib>

int check_valid_password(char *entry)
{
    char buffer[80] = {0}, policy_chr;
    short first, last;
    int contains_first = 0x0, contains_last = 0x0;

    // 1-3 a: abcde
    sscanf(entry, "%hd-%hd %c: %79s ", &first, &last, &policy_chr, buffer);

    if (buffer[first-1] == policy_chr) contains_first = 0x1;
    if (buffer[last-1] == policy_chr) contains_last = 0x1;

    // Following the policy, only one of the positions
    // must have the policy character.
    return contains_first ^ contains_last;
}

int main()
{
    char *line = NULL;
    size_t len = 0;
    int valid_pws = 0;

    while(getline(&line, &len, stdin) > 0)
    {
        valid_pws += check_valid_password(line) ? 1 : 0;
    }

    printf("%d\n", valid_pws);

    free(line);
}

