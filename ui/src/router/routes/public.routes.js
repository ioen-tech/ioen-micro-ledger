const routes = [
  {
    path: '/',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Home',
        component: () => import('@/views/home/Index.vue')
      }
    ]
  },
  {
    path: '/demo-setup',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Demo Setup',
        component: () => import('@/views/demo-setup/Index.vue')
      }
    ]
  }
]

export default routes
