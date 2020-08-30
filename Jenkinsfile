// this guarantees the node will use this template
def imageName = "dokubectl"
def registry = "https://dreg.vsix.me:9443"
def credential = "dreg-registry"
def label = "herald-build"
def tag = null
podTemplate(imagePullSecrets: [credential],label: label,idleMinutes: 30,
  containers: [
    containerTemplate(name: 'jnlp', image: 'dreg.vsix.me:9443/jnlp-docker:latest', args: '${computer.jnlpmac} ${computer.name}'),
    containerTemplate(name: 'rust-nightly', image: 'dreg.vsix.me:9443/rust_nightly:20200828-02', command: 'cat', ttyEnabled: true),
    ],
    volumes: [
    hostPathVolume(hostPath: '/var/run/docker.sock', mountPath: '/var/run/docker.sock'),
    persistentVolumeClaim(mountPath: '/workspace', claimName: 'herald-build-cache', readOnly: false)
    ]
      ) {
    node(label) {
            container("rust-nightly") {
                dir("/workspace/herald") {
                    stage('Checkout') {
                        checkout scm
                        tag = sh(returnStdout: true, script: "git describe --tags || echo 'none'").trim()
                        hash = sh(returnStdout: true, script: "git rev-parse --short HEAD").trim()
                    }
                    stage('Setup') {
                        sh '''
                        apt -y update
                        apt -y install zip
                        '''
                    }
                    stage('Test') {
                        sh '''
                        ./checktest.sh
                        '''
                    }
                }
            }
        }
}
