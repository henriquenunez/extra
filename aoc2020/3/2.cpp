#include <iostream>
#include <cstdio>
#include <cstdlib>
#include <cstring>

void print_map(char** map, int rows, int cols)
{
    for (int i = 0; i < rows ; i++)
    {
        for (int j = 0 ; j < cols ; j++)
        {
            printf("%c", map[i][j]);
        }
        printf("\n"); 
    }
}

/* Trees appear as '#' on the map*/
unsigned long count_trees(char **map, int rows, int cols, int row_slope, int col_slope)
{
    int temp_row = 0, temp_col = 0;
    size_t tree_count = 0;

    while(temp_row < rows)
    {
        if (map[temp_row][temp_col] == '#')
            tree_count++;

        temp_col = (temp_col + col_slope) % cols;
        temp_row += row_slope;
    }

    return tree_count;
}

int main()
{
    char *line = NULL;
    size_t line_len = 0;

    // Tree map.
    char **map = NULL;
    int cols, rows = 0;

/*     0 1 2 3 4
 * 0 ->
 * 1 ->
 * 2 ->
 * 3 ->
 * */

    while (getline(&line, &line_len, stdin) > 0)
    {
        char *temp;
        //printf("Line is : {%s}\n", line); 

        sscanf(line, "%ms ", &temp);

        rows++;
        cols = strlen(temp);

        map = (char**) reallocarray(map, rows, sizeof(char**));
        map[rows-1] = temp;
    }

    //print_map(map, rows, cols);

    // "lf"
    printf("%lu\n",
            count_trees(map, rows, cols, 1, 1) *
            count_trees(map, rows, cols, 1, 3) *
            count_trees(map, rows, cols, 1, 5) *
            count_trees(map, rows, cols, 1, 7) *
            count_trees(map, rows, cols, 2, 1)
            );

    for (int i = 0 ; i < rows ; i ++)
    {
        free(map[i]);
    }
    free(map);

    free(line);

    return 0;
}

