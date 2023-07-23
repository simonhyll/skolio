<template>
    <div class="background">
        <div class="pageArea">
          <NuxtPage></NuxtPage>
        </div>
    </div>
</template>

<style scoped>
.background {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    flex-grow: 1;
    justify-content: center;
    align-items: center;
    background: rgba(0,0,0,0.3);
}
.pageArea {
    display: block;
    flex-direction: column;
    overflow: hidden;
    width: auto;
    height: auto;
    background: #fff;
    border-radius: 6px;
}
</style>
