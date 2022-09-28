def texture_path(t)
  case t
  when "Spawn"
    "spawn_32x32.png"
  when "Exit"
    "exit_32x32.png"
  when "ExitPrelude"
    "exit_prelude_32x32.png"
  when "Clip"
    "wall_32x32.png"
  when "Brush"
    "path_32x32.png"
  when "PortalA"
    "portal_blue_32x32.png"
  when "PortalB"
    "portal_orange_32x32.png"
  else
    "wall_32x32.png"
  end
end

out = "match id {\n"

Dir.glob("*.prefab").sort.each do |fn|
  out += "    \"#{fn.gsub(/\.prefab/, '')}\" => Room {\n        id: \"#{fn.gsub(/\.prefab/, '')}\".to_string(),\n        tiles: vec![\n"
  contents = File.read(fn)

  pos = [0, 0]
  exits = []

  contents.split("\n").each do |line|
    tiles = line.scan /([\{\[]\w+[\]\}])/

    if tiles
      tiles.flatten.each do |t|
        m = t.match /([\{\[])(\w+)/
        next unless m
        bracket = m[1]
        tiletype = m[2]

        if bracket == "{"
          exits.push [pos[0], pos[1]]
        end

        out += "            Tile { pos: Vec2 { x: #{pos[0]}.0, y: #{pos[1]}.0 }, tile_type: TileType::#{tiletype}, texture_path: \"#{texture_path(tiletype)}\".to_string() },\n"
        pos[0] += 1
      end
    end

    pos[0] = 0
    pos[1] -= 1
  end

  out += "        ],\n         exits: vec![\n"
  out += exits.map { |e| "            Vec2 { x: #{e[0]}.0, y: #{e[1]}.0 }," }.join("\n")
  out += "\n        ],\n        ..default()\n"
  out += "    },\n"
end

out += "    _ => Room::default(),\n}\n"

puts out
