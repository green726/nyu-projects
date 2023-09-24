### file generated by plotpy
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.ticker as tck
import matplotlib.patches as pat
import matplotlib.path as pth
import matplotlib.patheffects as pff
import matplotlib.lines as lns
import matplotlib.transforms as tra
import mpl_toolkits.mplot3d as m3d
NaN = np.NaN
EXTRA_ARTISTS = []
def add_to_ea(obj):
    if obj!=None: EXTRA_ARTISTS.append(obj)
COLORMAPS = [plt.cm.bwr, plt.cm.RdBu, plt.cm.hsv, plt.cm.jet, plt.cm.terrain, plt.cm.pink, plt.cm.Greys]
def get_colormap(idx): return COLORMAPS[idx % len(COLORMAPS)]
AX3D = None
def maybe_create_ax3d():
    global AX3D
    if AX3D == None:
        AX3D = plt.gcf().add_subplot(111, projection='3d')
        AX3D.set_xlabel('x')
        AX3D.set_ylabel('y')
        AX3D.set_zlabel('z')
        add_to_ea(AX3D)
def data_to_axis(coords):
    plt.axis() # must call this first
    return plt.gca().transLimits.transform(coords)
def axis_to_data(coords):
    plt.axis() # must call this first
    return plt.gca().transLimits.inverted().transform(coords)
def set_equal_axes():
    ax = plt.gca()
    if AX3D == None:
        ax.axes.set_aspect('equal')
        return
    try:
        ax.set_box_aspect([1,1,1])
        limits = np.array([ax.get_xlim3d(), ax.get_ylim3d(), ax.get_zlim3d()])
        origin = np.mean(limits, axis=1)
        radius = 0.5 * np.max(np.abs(limits[:, 1] - limits[:, 0]))
        x, y, z = origin
        ax.set_xlim3d([x - radius, x + radius])
        ax.set_ylim3d([y - radius, y + radius])
        ax.set_zlim3d([z - radius, z + radius])
    except:
        import matplotlib
        print('VERSION of MATPLOTLIB = {}'.format(matplotlib.__version__))
        print('ERROR: set_box_aspect is missing in this version of Matplotlib')
plt.title(r'Max Energy vs Iteration')
x=np.array([],dtype=float)
y=np.array([],dtype=float)
plt.plot(x,y,linewidth=2)
plt.gca().set_axisbelow(True)
plt.grid(linestyle='--',color='grey',zorder=-1000)
plt.xlabel(r'Iteration')
plt.ylabel(r'Max Energy')
h,l=plt.gca().get_legend_handles_labels()
if len(h)>0 and len(l)>0:
    leg=plt.legend(handlelength=3,ncol=1,loc='best')
    add_to_ea(leg)

fn=r'./plot.svg'
plt.savefig(fn,bbox_inches='tight',bbox_extra_artists=EXTRA_ARTISTS)
plt.show()
