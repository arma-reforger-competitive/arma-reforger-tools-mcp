---
title: Arma Reforger:Enfusion Blender Tools: NLA Strips Baking Tool
url: https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_NLA_Strips_Baking_Tool
---

## Description

NLA Strips Baking tools is a small tool useful for retargeting animations via NLA workflow. For instance, it is possible to create one large track containing existing animations and then create one track on top of it in **combine** mode to make adjustments to it. Afterwards, such adjusted tracks can be baked to respective action which can be easily exported by TXA exporter.

## Usage

Usage of tool is pretty simple and bake can be initiated by following few basic steps:

1. Click on **NLA Tracks Fetcher** icon
1. Select NLA Track which you want to bake in the field next to **NLA Tracks Fetcher** button (*i.e. Erected*)
1. Click on **Bake NLA Strips** button

After pressing that button, **all strips** in selected NLA track will be baked to new actions, which are using strip name + " Baked" suffix. Baking of actions might take some time depending on length of involved strips so please be patitent.

Once that process is completed, actions can be exported via TXA exporter

### Selected Strips Baking

For faster iteration, it is also possible to use **Bake selected strips only** option. When activated, a new section in the interface will become available where it is possible to specify which strips should be baked. Once list is filled, it is possible to click on **Bake NLA Strips** button to initiate baking process
