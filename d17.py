from heapq import heappush, heappop

file = open("./inputs/2023/17.txt").readlines()
grid = [list(int(y) for y in list(x.strip())) for x in file]


def add(prev, cur, queue, heat_loss: int, row: int, col: int, dr: int, dc: int, steps: int = 1):
	new_row = row + dr
	new_col = col + dc

	if not (0 <= new_row < len(grid) and 0 <= new_col < len(grid[new_row])):
		return

	next = (
		heat_loss + grid[new_row][new_col],
		new_row,
		new_col,
		dr,
		dc,
		steps,
	)
	prev[next] = cur

	heappush(
		queue,
		next,
	)


visited = set()
prev = dict()
priority_queue = [(0, 0, 0, 0, 0, 0)]

while priority_queue:
	cur = heappop(priority_queue)
	heat_loss, row, col, dr, dc, steps = cur

	if row == len(grid) - 1 and col == len(grid[row]) - 1:
		result = [[str(c) for c in row] for row in grid]
		current = cur
		while current:
			result[current[2]][current[1]] = {(0, 0): result[current[2]][current[1]], (1, 0): '<', (-1, 0): '>', (0, 1): 'v', (0, -1): '^'}[(current[3], current[4])]
			current = prev.get(current)

		for row in result:
			print("".join([str(c) for c in row]))
		print(heat_loss)
		break

	if (row, col, dr, dc, steps) in visited:
		continue

	visited.add((row, col, dr, dc, steps))

	if steps < 3 and (dr, dc) != (0, 0):
		add(prev, cur, priority_queue, heat_loss, row, col, dr, dc, steps + 1)

	for new_dr, new_dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
		if (new_dr, new_dc) != (dr, dc) and (new_dr, new_dc) != (-dr, -dc):
			add(prev, cur, priority_queue, heat_loss, row, col, new_dr, new_dc)