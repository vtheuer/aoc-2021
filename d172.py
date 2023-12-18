data = open("inputs/2023/17.txt", "r").read().splitlines()

from heapq import heappush,heappop

def genNeighbours(node,data,ultra):
    x,y,v,dir = node
    dx,dy = dir
    left,right = (-dy,dx),(dy,-dx)
    if v < (10 if ultra else 3) and 0<=x+dx<len(data) and 0<=y+dy<len(data[0]):
        yield (x+dx,y+dy,v+1,dir), int(data[x+dx][y+dy])
    for dx,dy in left,right:
        if 0<=x+dx<len(data) and 0<=y+dy<len(data[0]) and (not ultra or v > 3):
            yield (x+dx,y+dy,1,(dx,dy)), int(data[x+dx][y+dy])
    
    

def pathfind(data,ultra):
    Q_visited = set()
    start1,start2 = (0,0,0,(1,0)),(0,0,0,(0,1))
    dist = {start1:0,start2:0}
    Q = [(0,start1),(0,start2)]
    target = (len(data)-1,len(data[0])-1)
    while len(Q):
        _,u = heappop(Q)
        if u in Q_visited: continue #Outdated entry
        Q_visited.add(u)
        if u[:2] == target and (not ultra or u[2] > 3):
            target = u
            break
        for v,cost in genNeighbours(u,data,ultra):
            if v in Q_visited: continue
            alt = dist[u] + cost
            if v not in dist or alt < dist[v]:              
                dist[v] = alt
                heappush(Q,(alt,v))
    print(dist[target])

pathfind(data,False)
pathfind(data,True)
