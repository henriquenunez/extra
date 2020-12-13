#include <vector>
#include <cstdio>
#include <cstdlib>

int check_valid_password(char *entry)
{
    char buffer[80] = {0}, policy_chr;
    short max, min;

    // 1-3 a: abcde
    sscanf(entry, "%hd-%hd %c: %79s ", &min, &max, &policy_chr, buffer);

    // Now, in the buffer, check if 
    // the number of the policy characters
    // are within the limit.
    int count = 0;
    for (int i = 0 ; i < 80 ; i++)
    {
        if (buffer[i] == policy_chr)
            count++;
    }

    if (count < min || count > max) return 0;

    return 1;
}

int main()
{
    char *line = NULL;
    size_t len = 0;
    int valid_pws = 0;

    while(getline(&line, &len, stdin) > 0)
    {
        valid_pws += check_valid_password(line);
    }

    printf("%d\n", valid_pws);

    free(line);
}

