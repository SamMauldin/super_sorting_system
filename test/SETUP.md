# Setup

Running tests requires some setup:

## Java

Java 17+ is required.

## Spigot

Download the Spigot BuildTools jar [here](https://hub.spigotmc.org/jenkins/job/BuildTools/)
and place it at `test/server/BuildTools.jar`.

In the `test/server` directory, run `java -jar BuildTools.jar --rev 1.18.2`.

You will need to agree to the Minecraft EULA by copying `test/server/eula.txt.example`
to `test/server/eula.text` and setting `eula=true` in that file.

## WorldEdit

Download WorldEdit [here](https://dev.bukkit.org/projects/worldedit/files)
and place it at `test/server/plugins/WorldEdit.jar`.

## Rhino

Download Rhino runtime [here](https://github.com/mozilla/rhino/releases)
and place it at `test/server/plugins/WorldEdit/js.jar`.
