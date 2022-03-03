const routes = [
  {
    path: '/producer',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Producer',
        component: () => import('@/views/producer/Index.vue')
      }
    ]
  }
]

export default routes
