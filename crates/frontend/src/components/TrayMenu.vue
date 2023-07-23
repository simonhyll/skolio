<template>
    <div class="trayArea">
        <button
            v-for="item in items" 
            class="item" @click="$router.push(item.to)"
            :class="{'active':route.path === item.to}"
            @mouseenter="tooltipText = item.title" 
            @mouseleave="tooltipText = ''">
            <f :icon="item.icon" />
        </button>
        <div class="spacer"></div>
        <button
            class="item" @click="$router.push('settings')"
            :class="{'active':route.path === '/settings'}"
            @mouseenter="tooltipText = 'Settings'" 
            @mouseleave="tooltipText = ''">
            <f icon="fa-solid fa-gear" />
        </button>
        <teleport to="body">
            <div class="tooltip-text" v-if="tooltipText"
        :style="tooltipPosition">{{ tooltipText }}</div>
        </teleport>
    </div>
</template>

<script setup>
import { ref } from 'vue';
const route = useRoute()
const tooltipText = ref('')
const tooltipPosition = ref({ top: '0px', left: '0px' })
function updateTooltipPosition(event) {
    tooltipPosition.value = {
        top: `${event.clientY}px`,
        left: `${event.clientX + 20}px`
    };
}

onMounted(async () => {
    await nextTick();
    document.addEventListener('mousemove', updateTooltipPosition);
});
const items = [{
    title: 'Today',
    to: '/',
    icon: 'fa-solid fa-list'
},{
    title: 'Teacher',
    to: '/teacher',
    icon: 'fa-solid fa-robot'
},{
    title: 'Subjects',
    to: '/subjects',
    icon: 'fa-solid fa-book'
}]
</script>

<style scoped>
.item {
    width: 45px;
    height: 45px;
    border: none;
    background: #009999;
    color: #fff;
    font-weight: bold;
    outline: 1px solid #007799;
    border-radius: 12px;
    margin: 5px;
    padding: 9px;
}
.item:hover {
    background: #007799;
    color: #fff;
}
.item:active {
    background: #fff;
    color: #007799;
}
.active {
    background: #007799 !important;
    color: #fff !important;
}
.trayArea {
    flex-grow: 1;
    display:flex;
    flex-direction: column;
}
</style>