async function subtask() {
  console.log("> > subtask"); // <3>
}

async function task() {
  console.log("> before subtask"); // <2>
  await subtask();
  console.log("> after subtask"); // <5>
}

(async function main() {
  console.log("before promise"); // <1>
  let promise = task();
  console.log("promise is created"); // <4>
  await promise;
  console.log("promise is awaited"); // <6>
})();
