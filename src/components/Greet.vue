<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const installed_revit = ref("");
const installed_addins = ref("");

let installed_version = []

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  installed_revit.value = await invoke("get_installed_revit_dirs");
  
  installed_addins.value = await invoke("get_installed_addins")

  installed_revit.value.forEach(element => {
    installed_version = element.split(' ')[1]
  });
  console.log(installed_version)
}
greet()
</script>



<template>
  
  <b> Found the following version of Revit Installed </b>
  <div v-for="item in installed_revit">
    <div style="border:solid;margin-left: 40%; margin-right:40%;margin-top:10px;">
      {{item}}
    </div>
  </div>
  <br>
  <b> Installed Addins </b>
  <div v-for="i in installed_addins" style="border:solid;margin-left: 40%; margin-right:40%;margin-top:10px;">
    {{ i.split('.')[0] }}
  </div>

 
  
</template>
