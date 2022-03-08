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
  },
  {
    path: '/generation-methods',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Methods',
        component: () => import('@/views/generation-methods/Index.vue')
      }
    ]
  }
]

export default routes
