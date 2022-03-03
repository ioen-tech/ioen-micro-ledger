const routes = [
  {
    path: '/consumer',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Consumer',
        component: () => import('@/views/consumer/Index.vue')
      }
    ]
  }
]

export default routes
