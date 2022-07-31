export interface Server {
    display_name: string,
    online: boolean,
    player_online: number,
    player_max: number,
}

export interface Member {
    avatar: string,
    name: string,
}