<script setup>
import { ref } from "vue"
import request from './request'
let str_input = ref("")
let str_output = ref([])
const request_input_str = () => {
  request.get("/?input=" + str_input.value.replace(/\s+/g, "")).then(function (res) {
    if (str_output.value.length > 5) {
      str_output.value.pop()
    }

    if (str_input.value.indexOf("=") !== -1) {

      if (str_input.value.indexOf(" = ") !== -1) {
        str_output.value.unshift(new Date().getTime().toString() + "|" + str_input.value)
      } else {
        let str = str_input.value.split("=")
        str_output.value.unshift(new Date().getTime().toString() + "|" + str[0].trim() + " = " + str[1].trim())
      }
    } else {
      str_output.value.unshift(new Date().getTime().toString() + "|" + str_input.value + " = " + res.str)
    }
    str_input.value = ""
  })
}

</script>

<template>
  <div style="display: flex; justify-content: center; align-items: center; height: 100%; flex-direction: column;">
    <div style="height:300px; display: flex; flex-direction: column-reverse; align-items: center;">
      <transition-group name="list">
        <p class="container__item" v-for="item in str_output" :key="item" style="height: 50px; font-size: 30pt;">

          {{ item.split("|")[1] }}
        </p>
      </transition-group>
    </div>
    <input v-model="str_input" />
    <button @click="request_input_str">Run!</button>
  </div>
</template>

<style scoped>
input {
  width: 60%;
  border: none;
  margin-top: 10px;
  font-size: 30pt;
  border-bottom: 4px solid #a6a6a6;
  background: rgba(255, 255, 255, 0);
  transition: all 0.4s ease;
}

input:focus {
  border-bottom-color: #007bff;
  outline: none;
}

button {
  width: 200px;
  height: 50px;
  /*圆角，半透明，金属光泽，按下后按钮被按下缩小的动画，按钮是#007bff色 */
  border-radius: 10px;
  background-color: #007bffb0;
  box-shadow: 3px 3px 10px #003874aa;
  border: none;
  color: #fff;
  font-size: 20pt;
  margin-top: 10px;
  transition: all 0.4s ease;
}

button:active {
  background-color: rgba(0, 0, 0, 0.8);
  box-shadow: 0 0 10px rgba(255, 255, 255, 0.8);
  transform: scale(0.95);
}

.container {
  display: flex;
  flex-wrap: wrap;
  flex-direction: column;
}

.list-enter-active,
.list-leave-active {
  transition: all 1s ease;
}

.list-enter,
.list-leave-to {
  opacity: 0;
  transform: translateY(-50px);
}
</style>
