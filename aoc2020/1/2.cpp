#include <cstdio>
#include <vector>
#include <cstdlib>

/*
 * 0    1   2   3   4   5   6   7
 * 0    1   0   1   1   0   0   0
 *
 * #define size_t unsigned int
 * */

//Part 2
int main()
{
    std::vector<int> input_vec;
    int temp;
    char *line = NULL;
    size_t line_len = 0;
    int temp_max = 0;
    int *freq_vec;

    while(getline(&line, &line_len, stdin) > 0)
    {
        sscanf(line, "%d", &temp);
        input_vec.push_back(temp);
        if (temp > temp_max) temp_max = temp;
    }

    freq_vec = (int*) calloc(temp_max + 1, sizeof(int));

    /*
     *  for (int i = 0 ; i < input_vec.size(); i++)
     *  {
     *      int val = input_vec[i];
     *  }
     *
     *
     * */

    for (int val : input_vec)
    {
        freq_vec[val] = 1;
    }

    //f_val -> first val, so we can check with
    //other 2 vals.
    //I know it's O(n^2), but i'm tired

    bool stop_searching = false;
    for (int f_val : input_vec)
    {
        int f_remainder = 2020 - f_val;

        for (int val : input_vec)
        {
            int remainder = f_remainder - val;

            if (remainder >= 0)
            if (freq_vec[remainder]) //Checks existence.
            {
                printf("%d", remainder * val * f_val);
                stop_searching = true;
                break;
            }
        }
        if (stop_searching) break;
    }

    free(freq_vec);
    free(line);

    return 0;
}
